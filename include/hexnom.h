#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Color {
  uint8_t red;
  uint8_t green;
  uint8_t blue;
};

template<typename T>
struct ResultCTransport {
  bool is_ok;
  char *err_msg;
  int err_len;
  T *data;
};

extern "C" {

/// Destructor for the error message strings return from ResultCTransport types
/// across the FFI.
void destroy_err_msg(char *err_msg);

ResultCTransport<Color> hex_color_c(char *input);

} // extern "C"
