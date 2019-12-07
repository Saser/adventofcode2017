#include "year2019/day06/day06.h"

#include <fstream>

#include "gtest/gtest.h"

TEST(Year2019Day06, Part1Example) {
  std::ifstream input("year2019/day06/testdata/example");
  std::string output = "42";
  adventofcode::answer_t a = day06::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day06, Part1Actual) {
  std::ifstream input("year2019/testdata/06");
  std::string output = "";
  adventofcode::answer_t a = day06::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}