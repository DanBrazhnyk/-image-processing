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

BitmapImage::~BitmapImage() {
    delete[] pixels;
}