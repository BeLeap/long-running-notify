const std = @import("std");

const stdout = std.io.getStdOut().writer();

const ip = std.x.net.ip;
const tcp = std.x.net.tcp;

const Buffer = std.x.os.Buffer;
const Socket = std.x.os.Socket;

pub fn send() anyerror!void {
    const client = try createClient();
    defer client.deinit();
    _ = try sendMessage(client, "EHLO running.notify\r\n");
    _ = try readMessage(client);
    _ = try sendMessage(client, "MAIL FROM: <long@running.notify>\r\n");
    _ = try readMessage(client);
    _ = try sendMessage(client, "RCPT TO: <changseo.jang@riiid.co>\r\n");
    _ = try readMessage(client);
    _ = try sendMessage(client, "DATA\r\n");
    _ = try readMessage(client);
    _ = try sendMessage(client, "Subject: Hello\r\n");
    _ = try sendMessage(client, "Message-ID: <msg@running.notify>\r\n");
    _ = try sendMessage(client, "From: <long@running.notify>\r\n");
    _ = try sendMessage(client, "To: <changseo.jang@riiid.co>\r\n\r\n");
    _ = try sendMessage(client, "merong3\r\n");
    _ = try sendMessage(client, ".\r\n");
    _ = try readMessage(client);
    _ = try sendMessage(client, "QUIT\r\n");
}

fn sendMessage(client: tcp.Client, message: []const u8) !usize {
  return try client.writeMessage(Socket.Message.fromBuffers(&[_]Buffer{Buffer.from(message)}), 0);
}

fn readMessage(client: tcp.Client) !void {
    const reader = client.reader(0);
    var buffer: [1024]u8 = [_]u8{0} ** 1024;
    _ = try reader.read(&buffer);
    try stdout.print("{s}\r\n", .{buffer});
}

fn createClient() !tcp.Client {
    const client = try tcp.Client.init(.ip, .{ .close_on_exec = true });
    const address = ip.Address.initIPv4(.{ .octets = [_]u8{ 64, 233, 188, 26 } }, 25);
    try client.connect(address);
    return client;
}

test "send" {
    try send();
}
