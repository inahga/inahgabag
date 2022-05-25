package goo

import (
	"fmt"

	"golang.org/x/exp/constraints"
)

func Partition[T constraints.Ordered](arr []T, n int) int {
	pivot := arr[n]
	smallest := 0

	for index, value := range arr {
		if value < pivot {
			arr[index], arr[smallest] = arr[smallest], arr[index]
			smallest++
		}
		fmt.Println(arr)
	}
	if n != 0 && smallest != len(arr)-1 {
		arr[smallest], arr[n] = arr[n], arr[smallest]
	}
	return smallest
}
