package main

import "fmt"
import "bufio"
import "os"

type Path struct {
	x int;
	y int;
}

func main() {
	area := make([]string, 0)

	s := bufio.NewScanner(os.Stdin)
    for s.Scan() {
		area = append(area, s.Text())
	}

	product := 1
	for _, path := range []Path {
		Path{x: 1, y: 1},
		Path{x: 3, y: 1},
		Path{x: 5, y: 1},
		Path{x: 7, y: 1},
		Path{x: 1, y: 2},
	} {
		product *= countTrees(&area, &path)
		fmt.Println(path, countTrees(&area, &path))
	}
	fmt.Println(product)
}

func countTrees(area *[]string, path *Path) int {
	x, y := 0, 0
	total := 0

	for y < len(*area) {
	  row := (*area)[y]
	  if row[x] == '#' {
		total += 1
	  }
	  x = (x + path.x) % len(row)
	  y += path.y
	}

	return total
}
