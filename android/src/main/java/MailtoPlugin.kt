package org.dashchat.mailto

import android.app.Activity
import android.content.Intent
import android.content.pm.PackageManager
import android.net.Uri
import android.webkit.WebView
import androidx.core.content.FileProvider
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import java.io.File

@InvokeArg
class MailtoArgs {
    lateinit var email: String
    var subject: String? = null
    var body: String? = null
    var attachments: Array<String>? = null
}

@TauriPlugin
class MailtoPlugin(private val activity: Activity) : Plugin(activity) {

    @Command
    fun mailto(invoke: Invoke) {
        val args = invoke.parseArgs(MailtoArgs::class.java)
        val attachmentUris = mutableListOf<Uri>()

        args.attachments?.forEach { path ->
            val file = File(path)
            if (file.exists()) {
                val uri = FileProvider.getUriForFile(
                    activity,
                    "${activity.applicationContext.packageName}.mailto.fileprovider",
                    file
                )
                attachmentUris.add(uri)
            }
        }

        val intent = if (attachmentUris.size > 1) {
            Intent(Intent.ACTION_SEND_MULTIPLE).apply {
                type = "message/rfc822"
                putExtra(Intent.EXTRA_EMAIL, arrayOf(args.email))
                args.subject?.let { putExtra(Intent.EXTRA_SUBJECT, it) }
                args.body?.let { putExtra(Intent.EXTRA_TEXT, it) }
                putParcelableArrayListExtra(Intent.EXTRA_STREAM, ArrayList(attachmentUris))
                addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
            }
        } else {
            Intent(Intent.ACTION_SEND).apply {
                type = "message/rfc822"
                putExtra(Intent.EXTRA_EMAIL, arrayOf(args.email))
                args.subject?.let { putExtra(Intent.EXTRA_SUBJECT, it) }
                args.body?.let { putExtra(Intent.EXTRA_TEXT, it) }
                if (attachmentUris.size == 1) {
                    putExtra(Intent.EXTRA_STREAM, attachmentUris[0])
                    addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
                }
            }
        }

        // Query which packages handle mailto: to filter chooser to email apps only
        val emailIntent = Intent(Intent.ACTION_SENDTO, Uri.parse("mailto:"))
        val emailApps = activity.packageManager.queryIntentActivities(emailIntent, PackageManager.MATCH_DEFAULT_ONLY)

        if (emailApps.isEmpty()) {
            invoke.reject("No email apps found")
            return
        }

        // Create a targeted send intent for each email app
        val targetedIntents = emailApps.map { resolveInfo ->
            Intent(intent).apply {
                setPackage(resolveInfo.activityInfo.packageName)
            }
        }

        val chooser = Intent.createChooser(targetedIntents.first(), null).apply {
            putExtra(Intent.EXTRA_INITIAL_INTENTS, targetedIntents.drop(1).toTypedArray())
            addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
        }

        activity.startActivity(chooser)
        invoke.resolve()
    }
}
