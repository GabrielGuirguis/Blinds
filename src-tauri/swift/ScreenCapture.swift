import ScreenCaptureKit
import Foundation
import Darwin

@_cdecl("hello_world")
public func hello_world() -> Bool {
    return true;
}

struct WindowData: Codable {
    let id: String
    let name: String,
    let app_id: String,
    let app_name: String,
    let is_hidden: bool,
}

func getWindowDataAsync() async throws -> [WindowData] {
    let content = try await SCShareableContent.excludingDesktopWindows(false, onScreenWindowsOnly: false)

    var windowsData: [WindowData] = []

    for window in content.window 
}

@_cdecl("request_screen_capture_access")
public func requestScreenCaptureAccess() -> Bool {
    // macOS 15+ logic might differ, but basic check:
    let stream = CGDisplayStream(
        dispatchQueueDisplay: 1,
        outputWidth: 100,
        outputHeight: 100,
        pixelFormat: 1111970369, // BGRA
        properties: nil,
        queue: DispatchQueue.main,
        handler: { _, _, _, _ in }
    )
    return stream != nil
}

@_cdecl("get_window_data")
public func getWindowData() -> UnsafeMutablePointer<CChar>? {
    let windows = []

    let availableWindows = SCShareableContent.get

    let encoder = JSONEncoder()
    guard let data = try? encoder.encode(windows) else { return nil }
    let count = data.count

    guard let raw = malloc(count + 1) else { return nil }
    data.withUnsafeBytes { (srcPtr: UnsafeRawBufferPointer) in
        memcpy(raw, srcPtr.baseAddress!, count)
    }

    raw.storeBytes(of: UInt8(0), toByteOffset: count, as: UInt8.self)
    return UnsafeMutablePointer<CChar>(raw.assumingMemoryBound(to: CChar.self))
}

@_cdecl("free_swift_buffer")
public func freeSwiftBuffer(_ ptr: UnsafeMutablePointer<CChar>?) {
    if let p = ptr { free(p) }
}