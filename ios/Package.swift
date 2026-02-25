// swift-tools-version:5.7
import PackageDescription

let package = Package(
    name: "tauri-plugin-mailto",
    platforms: [
        .iOS(.v13)
    ],
    products: [
        .library(
            name: "tauri-plugin-mailto",
            type: .static,
            targets: ["tauri-plugin-mailto"]
        ),
    ],
    dependencies: [
        .package(name: "Tauri", path: "../.tauri/tauri-api")
    ],
    targets: [
        .target(
            name: "tauri-plugin-mailto",
            dependencies: [
                .product(name: "Tauri", package: "Tauri")
            ],
            path: "Sources"
        ),
    ]
)
