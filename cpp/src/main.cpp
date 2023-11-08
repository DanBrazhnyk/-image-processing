#include "BitmapImage.h"
#include <iostream>

using namespace std;

int main() {
    BitmapImage bmpImage("image-03.bmp");
    cout << "w: " << bmpImage.width << ", h: " << bmpImage.height << endl;

    Pixel pixel = bmpImage.pixels[1131 * bmpImage.width + 1131];
    cout << "Red: " << static_cast<int>(pixel.red)
        << ", Green: " << static_cast<int>(pixel.green)
        << ", Blue: " << static_cast<int>(pixel.blue) 
        << ", alpha: " << static_cast<int>(pixel.alpha) << endl;
   

    return 0;
}
