#include "year2019/day13/day13.h"

#include <fstream>
#include <string>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day13, Part1Actual) {
  std::ifstream input("year2019/testdata/13");
  std::string output = "216";
  adventofcode::answer_t a = day13::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

// TEST(Year2019Day13, Part2Example1) {
//   std::istringstream input("some input here");
//   std::string output = "some output here";
//   adventofcode::answer_t a = day13::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
// }

// TEST(Year2019Day13, Part2Actual) {
//   std::ifstream input("year2019/testdata/13");
//   std::string output = "some output here";
//   adventofcode::answer_t a = day13::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
//   input.close();
// }