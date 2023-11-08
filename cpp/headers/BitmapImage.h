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
    ~BitmapImage();

    Pixel getPixel(int x, int y) const;

    int width, height;
    Pixel* pixels;
};