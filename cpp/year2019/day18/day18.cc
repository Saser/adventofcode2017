#include "year2019/day18/day18.h"

#include <istream>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "adventofcode.h"

using raw_grid_t = std::vector<std::vector<char>>;
using row_i_t = raw_grid_t::size_type;
using col_i_t = raw_grid_t::value_type::size_type;
using point_t = std::pair<row_i_t, col_i_t>;

struct grid_t {
  raw_grid_t g;

  const char& at(const point_t& point) const;
  point_t start() const;
  std::set<point_t> neighbors(const point_t& point) const;
};

adventofcode::answer_t solve(std::istream& is, int part);
grid_t parse(std::istream& is);

bool is_wall(char c);
bool is_key(char c);
bool is_door(char c);

namespace day18 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  return adventofcode::err("not implemented yet");
}

grid_t parse(std::istream& is) {
  raw_grid_t g;
  std::string line;
  while (std::getline(is, line)) {
    g.push_back(std::vector<char>(line.begin(), line.end()));
  }
  return grid_t {g};
}

bool is_wall(char c) {
  return c == '#';
}

bool is_key(char c) {
  return c >= 'a' && c <= 'z';
}

bool is_door(char c) {
  return c >= 'A' && c <= 'Z';
}

const char& grid_t::at(const point_t& point) const {
  auto [row_i, col_i] = point;
  return g.at(row_i).at(col_i);
}

point_t grid_t::start() const {
  for (row_i_t row_i = 0; row_i < g.size(); row_i++) {
    for (col_i_t col_i = 0; col_i < g.at(row_i).size(); col_i++) {
      if (g.at(row_i).at(col_i) == '@') {
        return {row_i, col_i};
      }
    }
  }
  return {0, 0};
}

std::set<point_t> grid_t::neighbors(const point_t& point) const {
  std::set<point_t> n;
  auto [row_i, col_i] = point;
  if (row_i > 0) {
    n.insert({row_i - 1, col_i});
  }
  if (row_i < g.size()) {
    n.insert({row_i + 1, col_i});
  }
  if (col_i > 0) {
    n.insert({row_i, col_i - 1});
  }
  if (col_i < g.at(0).size()) {
    n.insert({row_i, col_i + 1});
  }
  return n;
}
