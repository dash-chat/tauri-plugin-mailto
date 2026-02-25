import MessageUI
import SwiftRs
import Tauri
import UIKit
import UniformTypeIdentifiers
import WebKit

struct MailtoArgs: Decodable {
    let email: String
    let subject: String?
    let body: String?
    let attachments: [String]?
}

class MailtoPlugin: Plugin, MFMailComposeViewControllerDelegate {
    private var pendingInvoke: Invoke?

    @objc public func mailto(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(MailtoArgs.self)

        DispatchQueue.main.async {
            if MFMailComposeViewController.canSendMail() {
                self.presentMailCompose(invoke: invoke, args: args)
            } else {
                self.openMailtoUrl(invoke: invoke, args: args)
            }
        }
    }

    private func presentMailCompose(invoke: Invoke, args: MailtoArgs) {
        let composer = MFMailComposeViewController()
        composer.mailComposeDelegate = self
        composer.setToRecipients([args.email])

        if let subject = args.subject {
            composer.setSubject(subject)
        }
        if let body = args.body {
            composer.setMessageBody(body, isHTML: false)
        }

        if let attachments = args.attachments {
            for path in attachments {
                let url = URL(fileURLWithPath: path)
                if let data = try? Data(contentsOf: url) {
                    let mimeType = mimeTypeForPath(path)
                    let filename = url.lastPathComponent
                    composer.addAttachmentData(data, mimeType: mimeType, fileName: filename)
                }
            }
        }

        pendingInvoke = invoke

        guard let viewController = UIApplication.shared.keyWindow?.rootViewController else {
            invoke.resolve()
            return
        }

        var topController = viewController
        while let presented = topController.presentedViewController {
            topController = presented
        }

        topController.present(composer, animated: true)
    }

    private func openMailtoUrl(invoke: Invoke, args: MailtoArgs) {
        var components = URLComponents()
        components.scheme = "mailto"
        components.path = args.email

        var queryItems: [URLQueryItem] = []
        if let subject = args.subject {
            queryItems.append(URLQueryItem(name: "subject", value: subject))
        }
        if let body = args.body {
            queryItems.append(URLQueryItem(name: "body", value: body))
        }
        if !queryItems.isEmpty {
            components.queryItems = queryItems
        }

        if let url = components.url {
            UIApplication.shared.open(url)
        }

        invoke.resolve()
    }

    public func mailComposeController(
        _ controller: MFMailComposeViewController,
        didFinishWith result: MFMailComposeResult,
        error: Error?
    ) {
        controller.dismiss(animated: true)
        pendingInvoke?.resolve()
        pendingInvoke = nil
    }

    private func mimeTypeForPath(_ path: String) -> String {
        let url = URL(fileURLWithPath: path)
        let ext = url.pathExtension.lowercased()

        if #available(iOS 14.0, *) {
            if let utType = UTType(filenameExtension: ext) {
                return utType.preferredMIMEType ?? "application/octet-stream"
            }
        }

        // Fallback for iOS 13
        let mimeTypes: [String: String] = [
            "pdf": "application/pdf",
            "png": "image/png",
            "jpg": "image/jpeg",
            "jpeg": "image/jpeg",
            "gif": "image/gif",
            "txt": "text/plain",
            "html": "text/html",
            "zip": "application/zip",
            "doc": "application/msword",
            "docx": "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "xls": "application/vnd.ms-excel",
            "xlsx": "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            "csv": "text/csv",
        ]

        return mimeTypes[ext] ?? "application/octet-stream"
    }
}

@_cdecl("init_plugin_mailto")
func initPlugin() -> Plugin {
    return MailtoPlugin()
}
