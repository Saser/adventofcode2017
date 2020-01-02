package day25

import (
	"testing"

	"github.com/Saser/adventofcode/internal/testcase"
)

const inputFile = "../testdata/25"

func TestPart1(t *testing.T) {
	for _, tc := range []testcase.TestCase{
		testcase.FromString("example", "To continue, please consult the code grid in the manual.  Enter the code at row 2, column 1.", "31916031"),
		testcase.FromFile(t, inputFile, "19980801"),
	} {
		testcase.Run(t, tc, Part1)
	}
}

func BenchmarkPart1(b *testing.B) {
	tc := testcase.FromFile(b, inputFile, "")
	testcase.Bench(b, tc, Part1)
}
