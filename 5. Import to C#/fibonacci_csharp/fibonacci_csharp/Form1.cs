using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using System.Runtime.InteropServices;

namespace fibonacci_csharp
{
    public partial class Form1 : Form
    {
        [DllImport("fibonacci.dll", EntryPoint = "fibonacci")]
        public static extern int fibonacci(int Index);
        public Form1()
        {
            InitializeComponent();
            comboBox1.SelectedIndex = 0;
        }

        private void button1_Click(object sender, EventArgs e)
        {
            System.Diagnostics.Stopwatch sw = new System.Diagnostics.Stopwatch();
            var Index = Convert.ToInt32(comboBox1.SelectedItem);
            rust_result.Text = "-";
            csharp_result.Text = "-";
            sw.Start();
            fibonacci(Index);
            sw.Stop();
            rust_result.Text = sw.ElapsedMilliseconds.ToString();

            sw.Restart();
            csharp_fibonacci(Index);
            sw.Stop();
            csharp_result.Text = sw.ElapsedMilliseconds.ToString();
        }
        public int csharp_fibonacci(int Index)
        {
            if (Index == 0) { return 0; }
            else if (Index == 1) { return 1; }
            else { return csharp_fibonacci(Index - 1) + csharp_fibonacci(Index - 2); }
        }
    }
}
