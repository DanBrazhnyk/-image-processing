#include "BitmapImage.h"
#include <fstream>
#include <iostream>

using namespace std;

BitmapImage::BitmapImage(const std::string& filename) : width(0), height(0), pixels(nullptr) {
    std::ifstream inImg(filename, std::ios::binary);
    if (!inImg) {
        std::cerr << "Unable to open image." << std::endl;
        exit(1);
    }
    inImg.read(reinterpret_cast<char*>(fileHeader), 14);

    inImg.read(reinterpret_cast<char*>(informationHeader), 108);

    int bitsPerPixel = informationHeader[14] | (informationHeader[15] << 8);
    if (bitsPerPixel != 32) {
        std::cerr << "This program only supports 32-bit BMP files." << std::endl;
        return;
    }

    width = informationHeader[4] | (informationHeader[5] << 8) | (informationHeader[6] << 16) | (informationHeader[7] << 24);
    height = informationHeader[8] | (informationHeader[9] << 8) | (informationHeader[10] << 16) | (informationHeader[11] << 24);
    pixels = new Pixel[width * height];
    inImg.read(reinterpret_cast<char*>(pixels), width * height * sizeof(Pixel));

    inImg.close();
}

void BitmapImage::computeCumulativeSums(int** sumRed, int** sumGreen, int** sumBlue) {
    for (int i = 0; i < height; ++i) {
        int rowSumRed = 0, rowSumGreen = 0, rowSumBlue = 0;
        for (int j = 0; j < width; ++j) {
            rowSumRed += pixels[i * width + j].red;
            rowSumGreen += pixels[i * width + j].green;
            rowSumBlue += pixels[i * width + j].blue;

            if (i == 0) {
                sumRed[i][j] = rowSumRed;
                sumGreen[i][j] = rowSumGreen;
                sumBlue[i][j] = rowSumBlue;
            }
            else {
                sumRed[i][j] = sumRed[i - 1][j] + rowSumRed;
                sumGreen[i][j] = sumGreen[i - 1][j] + rowSumGreen;
                sumBlue[i][j] = sumBlue[i - 1][j] + rowSumBlue;
            }
        }
    }
}

void BitmapImage::boxBlur(int radiusLength, char* outName) {

    if (radiusLength < 1) {
        std::cerr << "Radius length should be greater than 1." << std::endl;
        return;
    }
    int** sumRed = new int* [height];
    int** sumGreen = new int* [height];
    int** sumBlue = new int* [height];
    for (int i = 0; i < height; ++i) {
        sumRed[i] = new int[width];
        sumGreen[i] = new int[width];
        sumBlue[i] = new int[width];
    }

    computeCumulativeSums(sumRed, sumGreen, sumBlue);

    Pixel* new_pixels = new Pixel[width * height];
    int kernelSize = (2 * radiusLength + 1) * (2 * radiusLength + 1);

    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            int top = std::max(0, i - radiusLength);
            int bottom = std::min(height - 1, i + radiusLength);
            int left = std::max(0, j - radiusLength);
            int right = std::min(width - 1, j + radiusLength);

            int red_sum = sumRed[bottom][right] - sumRed[bottom][left] - sumRed[top][right] + sumRed[top][left];
            int green_sum = sumGreen[bottom][right] - sumGreen[bottom][left] - sumGreen[top][right] + sumGreen[top][left];
            int blue_sum = sumBlue[bottom][right] - sumBlue[bottom][left] - sumBlue[top][right] + sumBlue[top][left];

            new_pixels[i * width + j].red = red_sum / kernelSize;
            new_pixels[i * width + j].green = green_sum / kernelSize;
            new_pixels[i * width + j].blue = blue_sum / kernelSize;
            new_pixels[i * width + j].alpha = pixels[i * width + j].alpha;
        }
    }

    // Memory cleanup for sum arrays
    for (int i = 0; i < height; ++i) {
        delete[] sumRed[i];
        delete[] sumGreen[i];
        delete[] sumBlue[i];
    }
    delete[] sumRed;
    delete[] sumGreen;
    delete[] sumBlue;

    std::ofstream outImg(outName, std::ios::binary);

    if (!outImg) {
        std::cerr << "Can't save blurred image." << std::endl;
        return;
    }

    outImg.write(reinterpret_cast<const char*>(fileHeader), 14);
    outImg.write(reinterpret_cast<const char*>(informationHeader), 108);

    outImg.write(reinterpret_cast<const char*>(new_pixels), width * height * sizeof(Pixel));

    outImg.close();
}

void BitmapImage::merge(const BitmapImage& other, int newAlpha, char* outName) {

    if (width != other.width || height != other.height) {
        std::cerr << "Image should be the same size." << std::endl;
        return;
    }

    for (int i = 0; i < width * height; ++i) {
        pixels[i].alpha = newAlpha;
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

    std::ofstream outImg(outName, std::ios::binary);

    if (!outImg) {
        std::cerr << "Can't create new image." << std::endl;
        return;
    }

    outImg.write(reinterpret_cast<const char*>(fileHeader), 14);
    outImg.write(reinterpret_cast<const char*>(informationHeader), 108);

    outImg.write(reinterpret_cast<const char*>(new_pixels), width * height * sizeof(Pixel));

    outImg.close();
}

// Removes portion of image in shape of square with lower left corner coordinates (x,y)
void BitmapImage::removePortion(int x, int y, int sideLength, char* outName) {
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

    std::ofstream outImg(outName, std::ios::binary);

    if (!outImg) {
        std::cerr << "Can't create new image." << std::endl;
        return;
    }

    outImg.write(reinterpret_cast<const char*>(fileHeader), 14);
    outImg.write(reinterpret_cast<const char*>(informationHeader), 108);

    outImg.write(reinterpret_cast<const char*>(pixels), width * height * sizeof(Pixel));

    outImg.close();
}

BitmapImage::~BitmapImage() {
    delete[] pixels;
}