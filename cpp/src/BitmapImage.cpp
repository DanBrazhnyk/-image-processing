#include "BitmapImage.h"
#include <fstream>
#include <iostream>

BitmapImage::BitmapImage(const std::string& filename) : width(0), height(0), pixels(nullptr) {
    std::ifstream file(filename, std::ios::binary);
    if (!file) {
        std::cerr << "Unable to open file" << std::endl;
        return;
    }

    file.seekg(10);
    int dataOffset;
    file.read(reinterpret_cast<char*>(&dataOffset), 4);

    file.seekg(18);
    file.read(reinterpret_cast<char*>(&width), 4);
    file.read(reinterpret_cast<char*>(&height), 4);

    file.seekg(dataOffset, std::ios::beg);

    pixels = new Pixel[width * height];
    file.read(reinterpret_cast<char*>(pixels), width * height * sizeof(Pixel));
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
BitmapImage::~BitmapImage() {
    delete[] pixels;
}
