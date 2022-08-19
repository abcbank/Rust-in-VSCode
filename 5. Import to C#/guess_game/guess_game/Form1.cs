using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace guess_game
{
    public partial class Form1 : Form
    {
        public enum CmpResult
        {
            Equal,
            Small,
            Big
        }
        [DllImport("guess_game_lib.dll", EntryPoint = "create_password")]
        public static extern int create_password(int min, int max);
        [DllImport("guess_game_lib.dll", EntryPoint = "compare_number")]
        public static extern int compare_number(int pw, int to_cmp);
        public int _Password = -1;
        public int Password { get => _Password;
            set
            {
                _Password = value;
                textBox4.Text = _Password.ToString();
            }
        }
        public Form1()
        {
            InitializeComponent();
        }

        private void checkBox1_CheckedChanged(object sender, EventArgs e)
        {
            if (checkBox1.Checked)
            {
                textBox4.PasswordChar = '*';
            }
            else
            {
                textBox4.PasswordChar = (char)0;
            }
        }

        private void button2_Click(object sender, EventArgs e)
        {
            Password = create_password(0, 100);
        }

        private void button1_Click(object sender, EventArgs e)
        {
            switch((CmpResult)compare_number(Password, Convert.ToInt32(textBox3.Text)))
            {
                case CmpResult.Equal:
                    //...
                    break;
                case CmpResult.Big:
                    //...
                    break;
                case CmpResult.Small:
                    //...
                    break;
            }
        }
    }
}
