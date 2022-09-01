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

  var skipped = arg_it.skip();
  if (skipped == false) {
    return;
  }

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

  var child_process = try std.ChildProcess.init(argv_list.items, allocator);
  defer child_process.deinit();

  var term = try std.ChildProcess.spawnAndWait(child_process);
  try stdout.print("ended with {s}\n", .{term});

  for (argv_list.items) |arg| {
    allocator.free(arg);
  }
}
