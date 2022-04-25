using System;

namespace FizzBuzz
{
  class Program
  {
    static void Main(string[] args)
    {
    Console.Write("How many numbers to display: ");
    string countstr = Console.ReadLine();
    int countnum = Convert.ToInt32(countstr);
    for (int i = 1; i <= countnum; i++) {
        if (i % 3 == 0 && i % 5 == 0) {
            Console.WriteLine("FizzBuzz");
        } else if (i % 3 == 0) {
            Console.WriteLine("Fizz");
        } else if (i % 5 == 0) {
            Console.WriteLine("Buzz");
        } else {
            Console.WriteLine(i);
        }
    }
    }
  }
}

