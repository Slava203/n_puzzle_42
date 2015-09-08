#include <queue>
#include <unordered_set>
#include <list>
#include "AStarSolver.hpp"
#include "Node.hpp"
#include "NodePool.hpp"

AStarSolver::~AStarSolver() {}

AStarSolver::AStarSolver(unsigned short **map, unsigned short **finalMap, int size, IHeuristic& heuristic)
	: _size(size), _pool(size), _openlist(&AStarSolver::less_node),
	  _closelist(100, &AStarSolver::hash_node, &AStarSolver::eq_node),
	  _firstNode(map, size),
	  _finalNode(finalMap, size), _heuristic(heuristic),
	  _totalStates(0), _maxStates(0)
{
	this->_firstNode.heuristic = this->_heuristic.distance(map);
	this->_openlist.push(&this->_firstNode);
	this->_nextNodes.reserve(4);
}

const Node&		AStarSolver::lastNode(void) { return (*this->_lastNode); }

size_t	AStarSolver::hash_node(const Node* node) {
	return (node->_hash);
}

bool	AStarSolver::eq_node(const Node* a, const Node* b) {
	return (*a == *b);
}

bool	AStarSolver::less_node(const Node* a, const Node* b) {
	if (a->distance == b->distance)
		return (a->cost < b->cost);
	return (a->distance > b->distance);
}

size_t	AStarSolver::getTotalStates () {
	return (this->_totalStates);
}

size_t	AStarSolver::getMaxStates() {
	return (this->_maxStates);
}

constNodes	AStarSolver::buildPath(Node* node) const
{
	constNodes	lst;

	node = *this->_closelist.find(node);
	while (node != NULL)
	{
		lst.push_front(node);
		node = node->parent;
	}
	return (lst);
}

constNodes	AStarSolver::buildPath(void) const
{
	return (this->buildPath(this->_lastNode));
}

constNodes	AStarSolver::buildMultiPath(const AStarSolver& fromStart, const AStarSolver& fromEnd) {
	constNodes	ret;
	if (collide(fromStart, fromEnd))
	{
		constNodes	tmp;

		tmp = fromEnd.buildPath(fromStart._lastNode);
		tmp.pop_back();
		tmp.reverse();
		ret = fromStart.buildPath();
		ret.splice(ret.end(), tmp);
	}
	else if (collide(fromEnd, fromStart))
	{
		ret = buildMultiPath(fromEnd, fromStart);
		ret.reverse();
	}
	return (ret);
}

void	AStarSolver::createNextNodes(int size, Node* topNode, NodePool& pool) {
	static Node::Square const	offsets[4] = {{1, 0}, {-1, 0}, {0, -1}, {0, 1}};
	Node::Square		curr_pos0 = topNode->pos0;
	Node*				node;

	this->_nextNodes.clear();
	for (int i = 0; i < 4; i++)
	{
		Node::Square checked;
		checked = curr_pos0 + offsets[i];
		if (checked.x >= 0 && checked.x < size && checked.y >= 0 && checked.y < size)
		{
			node = pool.newNode();
			*node = *topNode;
			*node->square(curr_pos0) = *node->square(checked);
			*node->square(checked) = 0;
			node->cost += 1;
			node->heuristic = this->_heuristic.distance(node->map);
			node->distance = node->cost + node->heuristic;
			node->pos0 = checked;
			node->parent = topNode;
			node->hash();
			this->_nextNodes.push_back(node);
		}
	}
}

bool	AStarSolver::solve(void)
{
	Node* topNode = this->_openlist.top();

	this->_openlist.pop();
	this->_lastNode = topNode;
	if (topNode->heuristic == 0 && *topNode == this->_finalNode)
	{
		this->_closelist.insert(topNode);
		return (false);
	}
	this->createNextNodes(this->_size, topNode, this->_pool);
	for (Node* node : this->_nextNodes)
	{
		if (this->_closelist.find(node) == this->_closelist.end())
		{
			this->_totalStates++;
			this->_openlist.push(node);
		}
		else
		{
			this->_pool.delNode(node);
		}
	}
	this->_closelist.insert(topNode);
	if (this->_openlist.size() + this->_closelist.size() > this->_maxStates)
	{
		this->_maxStates = this->_openlist.size() + this->_closelist.size();
	}
	return true;
}

/**
 * Create the final board.
 */
unsigned short	**AStarSolver::getSnailForm(unsigned short **map, int size) {
	unsigned short **newMap = new unsigned short*[size * size];
	int total_size = size * size;
	int x = 0;
	int y = 0;
	int ix = 1;
	int iy = 0;
	int	maxx = 0;
	int	maxy = 0;

	for (int i = 0; i < total_size; i++)
	{
		int nx = x + ix;
		int ny = y + iy;

		newMap[i] = &map[y][x];
		if (nx < 0 || nx >= size || (ix != 0 && (ix > 0 ? nx >= size + maxx : nx < 0 - maxx)))
		{
			if (nx <= 0 || (ix < 0 && nx < 0 - maxx))
				maxx -= 1;
			iy = ix;
			ix = 0;
		}
		else if (ny < 0 || ny >= size || (iy != 0 && (iy > 0 ? ny >= size + maxy : ny < 0 - maxy)))
		{
			if (ny >= size || (iy > 0 && ny >= size + maxy))
				maxy -= 1;
			ix = -iy;
			iy = 0;
		}
		x += ix;
		y += iy;
	}
	return (newMap);
}

bool	AStarSolver::isSolvable(unsigned short **map, int size) {
	unsigned short	**newMap = getSnailForm(map, size);
	int		total_size = size * size;
	int		count = 0;

	for (int i = 0; i < total_size - 1; i++)
	{
		for (int j = i + 1; j < total_size; j++)
		{
			if (*newMap[i] && *newMap[j] && *newMap[i] > *newMap[j])
				count++;
		}
	}
	return (count % 2 == 0);
}

/**
 * Return a board wich correspond to the final state.
 */
unsigned short	**AStarSolver::finalSolution(int size) {
	unsigned short	**newMap = new unsigned short*[size];
	int		total_size = size * size;
	unsigned short	**snailMap;

	newMap[0] = new unsigned short[size * size];
	for (int i = 1; i < size; i++)
	{
		newMap[i] = newMap[0] + size * i;
		memset(newMap[i], 0, size);
	}
	snailMap = getSnailForm(newMap, size);
	for (int i = 0; i < total_size - 1; i++)
	{
		*(snailMap[i]) = i + 1;
	}
	return (newMap);
}

unsigned short	**AStarSolver::genMap(size_t size, size_t swaps)
{
	int signedSize = size;
	unsigned short	**map = finalSolution(size);
	short			offsets[4][2] = {
		{1, 0}, {0, 1}, {-1, 0}, {0, -1}
	};
	short	pos0[2] = {(short)(signedSize / 2), (short)(size / 2 - (size % 2 == 0 ? 1 : 0))};

	if (swaps == 0)
	{
		swaps = arc4random() % 1400;
	}
	while (swaps > 0)
	{
		int		off = arc4random() % 4;
		short	swapPosX = pos0[0] + offsets[off][0];
		short	swapPosY = pos0[1] + offsets[off][1];
		if (swapPosX >= 0 && swapPosX < signedSize && swapPosY >= 0 && swapPosY < signedSize)
		{
			map[pos0[0]][pos0[1]] = map[swapPosX][swapPosY];
			map[swapPosX][swapPosY] = 0;
			pos0[0] = swapPosX;
			pos0[1] = swapPosY;
		}
		swaps--;
	}
	return (map);
}

bool	AStarSolver::collide(const AStarSolver& a, const AStarSolver& b)
{
	auto	found = b._closelist.find(a._lastNode);

	if (found != b._closelist.end() )// && (*found)->cost - 6 <= a._lastNode->heuristic)
	{
		return (true);
	}
	return (false);
}
