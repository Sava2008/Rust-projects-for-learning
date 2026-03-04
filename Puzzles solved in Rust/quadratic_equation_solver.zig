const std = @import("std");
const builtin = @import("builtin");

pub fn main() void {
    std.debug.print("{}\n", .{quadratic_equation(1.0, -5.0, 0.0)});
}

fn quadratic_equation(a_quotient: f32, b_quotient: f32, c_quotient: f32) struct { ?f32, ?f32 } {
    if (a_quotient == 0.0) {
        if (b_quotient == 0.0) {
            return .{
                null,
                null,
            };
        }
        return .{
            -c_quotient / b_quotient,
            null,
        };
    }
    if (c_quotient == 0.0) {
        return .{
            -b_quotient / a_quotient,
            0.0,
        };
    }
    const discriminant: f32 = b_quotient * b_quotient - 4 * a_quotient * c_quotient;
    const double_a = 2 * a_quotient;
    if (discriminant < 0) {
        return .{ null, null };
    } else if (discriminant == 0) {
        return .{
            -b_quotient / double_a,
            null,
        };
    }
    const discriminant_root = std.math.sqrt(discriminant);

    return .{
        (-b_quotient + discriminant_root) / double_a,
        (-b_quotient - discriminant_root) / double_a,
    };
}
