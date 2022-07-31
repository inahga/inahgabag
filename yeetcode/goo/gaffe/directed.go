package gaffe

type DirectedGraph[T comparable] struct {
	Nodes map[T][]*T
}

func (d *DirectedGraph[T]) DFS(fn func(*T) bool) {
	var (
		stack   []*T
		visited = map[T]struct{}{}
	)
	for node := range d.Nodes {
		if _, v := visited[node]; !v {
			stack = append(stack, &node)
			for len(stack) > 0 {
				n := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				visited[*n] = struct{}{}
				for _, edge := range d.Nodes[*n] {
					if _, v := visited[*edge]; !v {
						stack = append(stack, edge)
					}
				}
				if !fn(n) {
					return
				}
			}
		}
	}
}

func (d *DirectedGraph[T]) DFSFrom(from T, fn func(*T) bool) {
	var (
		stack   []*T
		visited = map[T]struct{}{}
	)
	stack = append(stack, &from)
	for len(stack) > 0 {
		n := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		visited[*n] = struct{}{}
		for _, edge := range d.Nodes[*n] {
			if _, v := visited[*edge]; !v {
				stack = append(stack, edge)
			}
		}
		if !fn(n) {
			return
		}
	}
}

func (d *DirectedGraph[T]) Some() T {
	var empty T
	for key := range d.Nodes {
		return key
	}
	return empty
}

func (d *DirectedGraph[T]) BFS(fn func(*T) bool) {
	var (
		queue   []*T
		visited = map[T]struct{}{}
	)
	for node := range d.Nodes {
		if _, v := visited[node]; !v {
			queue = append(queue, &node)
			for len(queue) > 0 {
				n := queue[0]
				queue = queue[1:]
				visited[*n] = struct{}{}
				for _, edge := range d.Nodes[*n] {
					if _, v := visited[*edge]; !v {
						queue = append(queue, edge)
					}
				}
				if ret := fn(n); !ret {
					return
				}
			}
		}
	}
}

func (d *DirectedGraph[T]) recursiveDFS(fn func(*T) bool, node *T, visited map[T]struct{}) {
	visited[*node] = struct{}{}
	if ret := fn(node); !ret {
		return
	}
	for _, edge := range d.Nodes[*node] {
		if _, v := visited[*edge]; !v {
			d.recursiveDFS(fn, edge, visited)
		}
	}
}

func (d *DirectedGraph[T]) RecursiveDFS(fn func(*T) bool) {
	visited := map[T]struct{}{}
	for node := range d.Nodes {
		node := node
		if _, v := visited[node]; !v {
			d.recursiveDFS(fn, &node, visited)
		}
	}
}

func (d *DirectedGraph[T]) topologicalSort(node *T, visited map[T]struct{}, ret *[]*T) {
	visited[*node] = struct{}{}
	for _, edge := range d.Nodes[*node] {
		if _, v := visited[*edge]; !v {
			d.topologicalSort(edge, visited, ret)
		}
	}
	*ret = append(*ret, node)
}

func (d *DirectedGraph[T]) TopologicalSort() []*T {
	if d.IsCyclic() {
		return nil
	}

	var (
		ret     []*T
		visited = map[T]struct{}{}
	)
	for node := range d.Nodes {
		node := node
		if _, v := visited[node]; !v {
			d.topologicalSort(&node, visited, &ret)
		}
	}
	for i := 0; i < len(ret)/2; i++ {
		ret[i], ret[len(ret)-1-i] = ret[len(ret)-1-i], ret[i]
	}
	return ret
}

func (d *DirectedGraph[T]) isCyclic(node *T, visited map[T]struct{}, path map[T]struct{}) bool {
	visited[*node] = struct{}{}
	path[*node] = struct{}{}
	for _, edge := range d.Nodes[*node] {
		if _, ok := path[*edge]; ok {
			return true
		}
		if _, v := visited[*edge]; !v {
			if d.isCyclic(edge, visited, path) {
				return true
			}
		}
	}
	delete(path, *node)
	return false
}

func (d *DirectedGraph[T]) IsCyclic() bool {
	visited := map[T]struct{}{}
	path := map[T]struct{}{}
	for node := range d.Nodes {
		node := node
		if _, v := visited[node]; !v {
			if d.isCyclic(&node, visited, path) {
				return true
			}
		}
	}
	return false
}
