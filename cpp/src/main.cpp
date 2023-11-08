#include "BitmapImage.h"
#include <iostream>

using namespace std;

int main() {
    BitmapImage bmpImage("image-01.bmp");
    cout << "w: " << bmpImage.width << ", h: " << bmpImage.height << endl;

    Pixel pixel = bmpImage.pixels[2 * bmpImage.width + 123];
    cout << "Red: " << static_cast<int>(pixel.red)
        << ", Green: " << static_cast<int>(pixel.green)
        << ", Blue: " << static_cast<int>(pixel.blue) << std::endl;
   

    return 0;
}