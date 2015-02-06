import java.util.*;
import java.io.*;
import java.math.*;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
class Solution {
	static boolean zero;
	static boolean change;
	static boolean first = true;

    public static void main(String args[]) {
        Scanner in = new Scanner(System.in);
        String message = in.nextLine();

        // Write an action using System.out.println()
        // To debug: System.err.println("Debug messages...");

		for (int i = 0, n = message.length(); i < n; i++) {
			encode((int) message.charAt(i));
		}
		System.out.println("");
    }

	static void encode(int b) {
		int[] result = new int[7];
		for(int i = 6; i >= 0; --i) {
			result[i] = b & 1;
			b = b >> 1; //Ignore the sign byte.
		}
		printResult(result);
	}

	static void printResult(int[] result) {
		for(int i = 0; i < 7; ++i) {
			if(first) {
				first = false;
				zero = (result[0] == 0);
				System.out.print("0");
				if(zero) {
					System.out.print("0");
				}
				System.out.print(" ");
			} else {
				change = zero?result[i] != 0:result[i] == 0;
				zero = (result[i] == 0);
				if(change) {
					System.out.print(" 0");
					if(zero) {
						System.out.print("0");
					}
					System.out.print(" ");
				}
			}
			System.out.print("0");
		}
	}
}