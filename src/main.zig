const std = @import("std");

const stdout = std.io.getStdOut().writer();

pub fn main() anyerror!void {
  var gpa = std.heap.GeneralPurposeAllocator(.{}){};
  const allocator = gpa.allocator();
  defer {
    const leaked = gpa.deinit();
    if (leaked) @panic("LEAK");
  }

  const process = std.process;
  var arg_it = process.args();
  defer arg_it.deinit();

  var argv_list = std.ArrayList([]u8).init(allocator);
  defer argv_list.deinit();

  while (true) {
    if (arg_it.next(allocator)) |maybe_arg| {
      var arg = maybe_arg catch |err| { 
        std.debug.print("unwrap arg failed", .{});
        return err;
      };

      try argv_list.append(arg);
    } else {
      break;
    }
  }

  for (argv_list.items) |arg| {
    try stdout.print("{s}\n", .{arg});
    defer allocator.free(arg);
  }
}
