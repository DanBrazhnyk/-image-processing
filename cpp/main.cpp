#include "BitmapImage.h"
#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    if (argc < 2) {
        cout << "Not enough arguments." << endl;
        return -1;
    }
    else if (strcmp(argv[1], "blur") == 0) {
        if (argc != 5) {
            cout << "Wrong number of arguments." << endl;
            return -1;
        }
        int radius = 0;
        for (int i = 0; argv[4][i] != '\0'; ++i) {
            char c = argv[4][i];
            if (c < '0' || c > '9') {
                std::cout << "Radius should be a number.";
                return 1;
            }
            radius = radius * 10 + (c - '0');
        }
        BitmapImage bmpImage(argv[2]);
        bmpImage.boxBlur(radius, argv[3]);
    }
    else if (strcmp(argv[1], "merge") == 0) {
        if (argc != 6) {
            cout << "Wrong number of arguments." << endl;
            return -1;
        }
        int new_alpha = 0;
        for (int i = 0; argv[5][i] != '\0'; ++i) {
            char c = argv[5][i];
            if (c < '0' || c > '9') {
                std::cout << "alpha should be a number.";
                return 1;
            }
            new_alpha = new_alpha * 10 + (c - '0');
        }
        BitmapImage bmpImage_01(argv[2]);
        BitmapImage bmpImage_02(argv[3]);
        bmpImage_01.merge(bmpImage_02, new_alpha, argv[4]);
    }
    else if (strcmp(argv[1], "cut") == 0) {
        if (argc != 7) {
            cout << "Wrong number of arguments." << endl;
            return -1;
        }
        int x = 0;
        for (int i = 0; argv[4][i] != '\0'; ++i) {
            char c = argv[4][i];
            if (c < '0' || c > '9') {
                std::cout << "coordinates should be a number.";
                return 1;
            }
            x = x * 10 + (c - '0');
        }
        int y = 0;
        for (int i = 0; argv[5][i] != '\0'; ++i) {
            char c = argv[5][i];
            if (c < '0' || c > '9') {
                std::cout << "coordinates should be a number.";
                return 1;
            }
            y = y * 10 + (c - '0');
        }
        int length = 0;
        for (int i = 0; argv[6][i] != '\0'; ++i) {
            char c = argv[6][i];
            if (c < '0' || c > '9') {
                std::cout << "Length should be a number.";
                return 1;
            }
            length = length * 10 + (c - '0');
        }
        BitmapImage bmpImage(argv[2]);
        bmpImage.removePortion(x, y, length, argv[3]);
    }
    else {
        cout << "Wrong operation.";
        return -1;
    }
    return 0;
}
