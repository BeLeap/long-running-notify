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

  while (true) {
    if (arg_it.next(allocator)) |maybe_arg| {
      var arg = maybe_arg catch |err| { 
        std.debug.print("unwrap arg failed", .{});
        return err;
      };

      defer allocator.free(arg);
      std.debug.print("{s} ", .{arg});
    } else {
      break;
    }
  }
}
