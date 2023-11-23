#include "BitmapImage.h"
#include <fstream>
#include <iostream>

using namespace std;

BitmapImage::BitmapImage(const std::string& filename) : width(0), height(0), pixels(nullptr) {
    std::ifstream file(filename, std::ios::binary);
    if (!file) {
        std::cerr << "Unable to open file" << std::endl;
        return;
    }
    file.read(reinterpret_cast<char*>(fileHeader), 14);

    file.read(reinterpret_cast<char*>(informationHeader), 40);

    width = informationHeader[4] | (informationHeader[5] << 8) | (informationHeader[6] << 16) | (informationHeader[7] << 24);
    height = informationHeader[8] | (informationHeader[9] << 8) | (informationHeader[10] << 16) | (informationHeader[11] << 24);

    pixels = new Pixel[width * height];
    file.read(reinterpret_cast<char*>(pixels), width * height * sizeof(Pixel));

    file.close();
}

void BitmapImage::blendWith(const BitmapImage& other) { 

    if (width != other.width || height != other.height) {
        std::cerr << "error" << std::endl;
        exit;  
    }

    Pixel* new_pixels = new Pixel[width * height];

    for (int i = 0; i < width * height; ++i) {

        float alpha_front = pixels[i].alpha / 255.0f;
        float alpha_back = other.pixels[i].alpha / 255.0f;
        float alpha_combined = alpha_front + alpha_back * (1.0f - alpha_front);

        new_pixels[i].red = static_cast<uint8_t>((other.pixels[i].red * alpha_front + pixels[i].red * alpha_back * (1.0f - alpha_front)) / alpha_combined);
        new_pixels[i].green = static_cast<uint8_t>((other.pixels[i].green * alpha_front + pixels[i].green * alpha_back * (1.0f - alpha_front)) / alpha_combined);
        new_pixels[i].blue = static_cast<uint8_t>((other.pixels[i].blue * alpha_front + pixels[i].blue * alpha_back * (1.0f - alpha_front)) / alpha_combined);
        new_pixels[i].alpha = static_cast<uint8_t>(alpha_combined * 255.0f);

    }

    std::cout << "Image created" << std::endl;

    // TESTING TRANSFORMED PIXEL
    Pixel pixel = new_pixels[0 * width + 0];
    std::cout << "Red: " << static_cast<int>(pixel.red)
        << ", Green: " << static_cast<int>(pixel.green)
        << ", Blue: " << static_cast<int>(pixel.blue)
        << ", alpha: " << static_cast<int>(pixel.alpha) << std::endl;
    return;
}

// Removes portion of image in shape of square with upper left corner coordinates (x,y)
void BitmapImage::removePortion(int x, int y, int sideLength) {
    if (x > width || x < 0 || y > height || y < 0) {
        cout << "Picked point has to be in the range of the image." << endl;
        return;
    }

    // Setting each pixel of square to alpha 0
    for (int i = 0; i < sideLength && y + i <= height; i++) {
        for (int j = 0; j < sideLength && x + j <= width; j++) {
            pixels[(x + j) + width * (y + i)].alpha = 0;
        }
    }
}

BitmapImage::~BitmapImage() {
    delete[] pixels;
}
