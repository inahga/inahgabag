package goo

import (
	"fmt"
	"testing"
)

func TestReverse(t *testing.T) {
	tests := []struct {
		input, expected string
	}{
		{
			input:    "foobar",
			expected: "raboof",
		},
		{
			input:    "fooba",
			expected: "aboof",
		},
	}
	for index, test := range tests {
		t.Run(fmt.Sprint(index), func(t *testing.T) {
			result := Reverse(test.input)
			if result != test.expected {
				t.Errorf("expected %s, got %s", test.expected, result)
			}
		})
	}
}
