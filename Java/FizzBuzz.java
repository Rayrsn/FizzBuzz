package Java;

import java.util.Scanner;

class Main {
  public static void main(String[] args) {
    try (Scanner myObj = new Scanner(System.in)) {
        System.out.print("How many numbers to display: ");

        String numcount = myObj.nextLine();
        // print out the numbers from 1 to numcount
        for (int i = 1; i <= Integer.parseInt(numcount); i++) {
            if (i % 3 == 0 && i % 5 == 0) {
                System.out.println("FizzBuzz");
            } else if (i % 3 == 0) {
                System.out.println("Fizz");
            } else if (i % 5 == 0) {
                System.out.println("Buzz");
            } else {
                System.out.println(i);
            }
        }
    }
  }
}
