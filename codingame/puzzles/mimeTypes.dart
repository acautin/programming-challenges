import 'dart:io';

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
void main() {
    List inputs;
	Map<String, String> mimeList = new Map<String, String>();
    int N = int.parse(stdin.readLineSync()); // Number of elements which make up the association table.
    int Q = int.parse(stdin.readLineSync()); // Number Q of file names to be analyzed.
	
    for (int i = 0; i < N; i++) {
        inputs = stdin.readLineSync().split(' ');
        String extension = inputs[0].toLowerCase(); // file extension
        String type = inputs[1]; // MIME type.
		mimeList[extension] = type;
    }
    for (int i = 0; i < Q; i++) {
		// For each of the Q filenames, display on a line the corresponding MIME type.
		// If there is no corresponding type, then display UNKNOWN.
        String FNAME = stdin.readLineSync(); // One file name per line.
        List divided = (FNAME.toLowerCase()).split(".");
		if(divided.length < 2) {
			print("UNKNOWN");
		} else {
			if(mimeList[divided.last] == null) {
				print("UNKNOWN");
			} else {
				print(mimeList[divided.last]);
			}
		}
    }
}