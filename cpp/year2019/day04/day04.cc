#include "year2019/day04/day04.h"

#include <istream>
#include <string>

#include "absl/strings/str_split.h"

#include "adventofcode.h"
#include "year2019/day04/day04_internal.h"

namespace day04 {
  adventofcode::answer_t part1(std::istream& is) {
    std::string input;
    std::getline(is, input);
    std::vector<std::string> parts = absl::StrSplit(input, "-");
    int start = std::stoi(parts[0]);
    int end = std::stoi(parts[1]);
    int count = 0;
    for (int password = start; password <= end; password++) {
      auto digits = day04::digits(password);
      if (day04::has_double(digits, false) && day04::non_decreasing(digits)) {
        count++;
      }
    }
    return adventofcode::ok(std::to_string(count));
  }
}