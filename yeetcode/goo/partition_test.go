package goo

import (
	"fmt"
	"testing"
)

func TestPartition(t *testing.T) {
	tests := []struct {
		arr            []int
		n              int
		expectedReturn int
	}{
		{
			arr:            []int{3, 6, 1, 7, 3, 4, 4, 3},
			n:              2,
			expectedReturn: 0,
		},
		{
			arr:            []int{3, 6, 1, 7, 2, 4, 4, 9},
			n:              0,
			expectedReturn: 2,
		},
		{
			arr:            []int{3, 6, 1, 7, 2, 12, 4, 9},
			n:              7,
			expectedReturn: 6,
		},
		{
			arr:            []int{3, 6, 1, 7, 2, 12, 4, 9},
			n:              5,
			expectedReturn: 7,
		},
		{
			arr:            []int{3, 6, 1, 7, 2, 12, 4, 9},
			n:              4,
			expectedReturn: 1,
		},
		{
			arr:            []int{3, 6, 1, 7, 2, 12, 4, 9, 16},
			n:              8,
			expectedReturn: 8,
		},
		{
			arr:            []int{1, 2, 3, 4, 5, 6, 7},
			n:              3,
			expectedReturn: 3,
		},
		{
			arr:            []int{1, 2},
			n:              1,
			expectedReturn: 1,
		},
		{
			arr:            []int{1, 2},
			n:              0,
			expectedReturn: 0,
		},
		{
			arr:            []int{1},
			n:              0,
			expectedReturn: 0,
		},
		{
			arr:            []int{2, 1},
			n:              0,
			expectedReturn: 1,
		},
		{
			arr:            []int{2, 1},
			n:              1,
			expectedReturn: 0,
		},
	}
	for index, test := range tests {
		t.Run(fmt.Sprint(index), func(t *testing.T) {
			orig := test.arr[test.n]
			t.Logf("before: %v", test.arr)
			p := Partition(test.arr, test.n)
			t.Logf("after: %v", test.arr)
			if test.expectedReturn != p {
				t.Fatalf("expected %d, got %d", test.expectedReturn, p)
			}
			if test.arr[p] != orig {
				t.Fatalf("pivot point is not in the expected position")
			}
			for i := 0; i < p; i++ {
				if test.arr[i] > test.arr[p] {
					t.Error()
				}
			}
			for i := p + 1; i < len(test.arr); i++ {
				if test.arr[i] < test.arr[p] {
					t.Error()
				}
			}
		})
	}
}
