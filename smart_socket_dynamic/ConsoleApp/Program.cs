using System.Runtime.InteropServices;

namespace ConsoleApp;

public static class Program
{
    [DllImport("smart_socket_lib.dll")] // Replace with the actual path to your dynamic library
    private static extern IntPtr create_smart_socket();

    [DllImport("smart_socket_lib.dll")] // Replace with the actual path to your dynamic library
    private static extern void delete_smart_socket(IntPtr ptr);

    [DllImport("smart_socket_lib.dll")]
    private static extern void smart_socket_turn_on(IntPtr smartSocket);

    [DllImport("smart_socket_lib.dll")]
    private static extern void smart_socket_turn_off(IntPtr smartSocket);

    [DllImport("smart_socket_lib.dll")]
    private static extern bool smart_socket_get_state(IntPtr smartSocket);

    [DllImport("smart_socket_lib.dll")]
    private static extern void smart_socket_set_power_consumption(IntPtr smartSocket, double power);

    [DllImport("smart_socket_lib.dll")]
    private static extern double smart_socket_get_power_consumption(IntPtr smartSocket);

    private static void Main()
    {
        var smartSocketPtr = create_smart_socket();

        smart_socket_turn_on(smartSocketPtr);
        smart_socket_set_power_consumption(smartSocketPtr, 100.0);

        Console.WriteLine("Socket is turned on: " + smart_socket_get_state(smartSocketPtr));
        Console.WriteLine("Power consumption: " + smart_socket_get_power_consumption(smartSocketPtr) + " W");

        smart_socket_turn_off(smartSocketPtr);
        delete_smart_socket(smartSocketPtr);
    }
}