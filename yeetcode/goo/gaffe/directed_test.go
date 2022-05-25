package gaffe

import (
	"fmt"
	"testing"
)

var (
	graphNode1  = 1
	graphNode2  = 2
	graphNode3  = 3
	graphNode4  = 4
	graphNode5  = 5
	graphNode6  = 6
	graphNode7  = 7
	graphNode8  = 8
	graphNode9  = 9
	graphNode10 = 10
)

func TestDirectedGraphSearch(t *testing.T) {
	tests := []struct {
		name     string
		graph    DirectedGraph[int]
		expected []int
	}{
		{
			name: "connected acyclical graph",
			graph: DirectedGraph[int]{
				Nodes: map[int][]*int{
					graphNode1: {&graphNode2, &graphNode8},
					graphNode2: {&graphNode3, &graphNode7},
					graphNode3: {&graphNode4, &graphNode5, &graphNode6},
					graphNode8: {&graphNode9},
				},
			},
		},
		{
			name: "connected cyclical graph",
			graph: DirectedGraph[int]{
				Nodes: map[int][]*int{
					graphNode1: {&graphNode2, &graphNode3},
					graphNode2: {&graphNode1},
				},
			},
		},
		{
			name: "disconnected acyclical graph",
			graph: DirectedGraph[int]{
				Nodes: map[int][]*int{
					graphNode1: {&graphNode2},
					graphNode3: {&graphNode4},
					graphNode5: {&graphNode6, &graphNode7, &graphNode8},
					graphNode8: {&graphNode1},
				},
			},
		},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			test.graph.DFS(func(n *int) bool {
				fmt.Println(*n)
				return true
			})
		})
		t.Run(test.name, func(t *testing.T) {
			test.graph.RecursiveDFS(func(n *int) bool {
				fmt.Println(*n)
				return true
			})
		})
		t.Run(test.name, func(t *testing.T) {
			test.graph.BFS(func(n *int) bool {
				fmt.Println(*n)
				return true
			})
		})
		t.Run(test.name, func(t *testing.T) {
			fmt.Println(test.graph.IsCyclic())
		})
	}
}

func TestDirectedGraphTopologicalSort(t *testing.T) {
	tests := []struct {
		name     string
		graph    DirectedGraph[int]
		expected []int
	}{
		{
			name: "connected acyclical graph",
			graph: DirectedGraph[int]{
				Nodes: map[int][]*int{
					graphNode1: {&graphNode2, &graphNode8},
					graphNode2: {&graphNode3, &graphNode7},
					graphNode3: {&graphNode4, &graphNode5, &graphNode6},
					graphNode8: {&graphNode9},
				},
			},
		},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			for _, n := range test.graph.TopologicalSort() {
				fmt.Println(*n)
			}
		})
	}
}
