package main

import "fmt"
import "bufio"
import "os"
import "strings"
import "regexp"
import "strconv"

type Edge struct {
	count   int
	bagName string
}

type Bag struct {
	name     string
	children []Edge
}

func main() {
	bags := make(map[string]Bag)

	re := regexp.MustCompile(`(\d+) ([^,\.]+) bags?`)

	s := bufio.NewScanner(os.Stdin)
	for s.Scan() {
		pair := strings.SplitN(s.Text(), " bags contain ", 2)
		name := pair[0]

		children := make([]Edge, 0)
		for _, match := range re.FindAllStringSubmatch(pair[1], -1) {
			count, _ := strconv.Atoi(match[1])
			children = append(children, Edge{
				count:   count,
				bagName: match[2],
			})
		}

		bags[name] = Bag{
			name:     name,
			children: children,
		}
	}

	goal := "shiny gold"
	total := 0
	results := map[string]bool{}

	for _, bag := range bags {
		if bag.name != goal && resolveBag(&bags, &results, goal, &bag) {
			total += 1
		}
	}

	//fmt.Println(bags)
	fmt.Println(total)
}

func resolveBag(bags *map[string]Bag, results *map[string]bool, goal string, bag *Bag) bool {
	cached, hasValue := (*results)[bag.name]

	if hasValue {
		return cached
	}

	if bag.name == goal {
		return true
	}

	hasGoal := false
	for _, edge := range bag.children {
		child := (*bags)[edge.bagName]
		hasGoal = hasGoal || resolveBag(bags, results, goal, &child)
	}

	(*results)[bag.name] = hasGoal
	return hasGoal
}
