using System;
using System.Runtime.InteropServices;

namespace RustIntegration
{
    internal static class Native
    {
        [DllImport("librust")]
        internal static extern StringHandle get_struct();

        [DllImport("librust")]
        internal static extern void set_struct(string str);

        [DllImport("librust")]
        internal static extern void dealloc(IntPtr str);
    }

    public class StringHandle : SafeHandle
    {
        public StringHandle() : base(IntPtr.Zero, true) { }

        public override bool IsInvalid => false;

        protected override bool ReleaseHandle()
        {
            Native.dealloc(handle);
            return true;
        }

        public override string ToString()
        {
            return Marshal.PtrToStringUTF8(handle);
        }
    }

    class Program
    {
        static void Main(string[] args)
        {
            var strct = Native.get_struct();
            var s = strct.ToString();
            Console.WriteLine($"C#: {s}");
            strct.Dispose();

            Console.Write("Rust: ");
            Native.set_struct(s);

            Console.ReadLine();
        }
    }
}
