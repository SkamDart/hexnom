#include <iostream>

#include "hexnom.h"

int main()
{
	char* color = "#2F14DF";
	ResultCTransport<Color> mc = hex_color_c(color);
    if (mc.is_ok)
    {
        auto data = mc.data;
        std::cout
            << std::hex
            << static_cast<int>(data->red)
            << static_cast<int>(data->green)
            << static_cast<int>(data->blue);
    }

	return 0;
}
