#pragma once

#include <string>

struct Pixel {
    uint8_t blue;
    uint8_t green;
    uint8_t red;
    uint8_t alpha;
};

class BitmapImage {
public:
    BitmapImage(const std::string& filename);
    void blendWith(const BitmapImage& other);
    void removePortion(int x, int y, int sideLength);
    ~BitmapImage();

    Pixel getPixel(int x, int y) const;

    int width, height;
    unsigned char fileHeader[14];
    unsigned char informationHeader[40];
    Pixel* pixels;
};
