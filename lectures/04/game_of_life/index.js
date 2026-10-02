import init, { Cell, Universe } from "./pkg/game_of_life.js";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const { memory } = await init();

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

const getIndex = (row, column) => row * width + column;

const drawGrid = () => {
	ctx.beginPath();
	ctx.strokeStyle = GRID_COLOR;

	for (let i = 0; i <= width; i++) {
		ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
		ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
	}

	for (let j = 0; j <= height; j++) {
		ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
		ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
	}

	ctx.stroke();
};

const drawCells = () => {
	const cellsPtr = universe.cells();
	const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

	ctx.beginPath();

	for (let row = 0; row < height; row++) {
		for (let col = 0; col < width; col++) {
			const idx = getIndex(row, col);

			ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

			ctx.fillRect(
				col * (CELL_SIZE + 1) + 1,
				row * (CELL_SIZE + 1) + 1,
				CELL_SIZE,
				CELL_SIZE,
			);
		}
	}

	ctx.stroke();
};

let animationId = null;

const isPaused = () => animationId === null;

const renderLoop = () => {
	universe.tick();

	drawGrid();
	drawCells();

	animationId = requestAnimationFrame(renderLoop);
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
	playPauseButton.textContent = "⏸";
	renderLoop();
};

const pause = () => {
	playPauseButton.textContent = "▶";
	cancelAnimationFrame(animationId);
	animationId = null;
};

playPauseButton.addEventListener("click", () => {
	if (isPaused()) {
		play();
	} else {
		pause();
	}
});

const clearButton = document.getElementById("clear");

clearButton.addEventListener("click", () => {
	universe.clear();

	drawGrid();
	drawCells();
});

const cellAtEvent = (event) => {
	const boundingRect = canvas.getBoundingClientRect();

	const scaleX = canvas.width / boundingRect.width;
	const scaleY = canvas.height / boundingRect.height;

	const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
	const canvasTop = (event.clientY - boundingRect.top) * scaleY;

	const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
	const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

	return [row, col];
};

// Bresenham's line algorithm, so fast drags don't leave gaps between cells.
const forEachCellOnLine = (row0, col0, row1, col1, callback) => {
	let row = row0;
	let col = col0;

	const dRow = Math.abs(row1 - row0);
	const dCol = Math.abs(col1 - col0);
	const stepRow = row1 > row0 ? 1 : -1;
	const stepCol = col1 > col0 ? 1 : -1;

	let err = dRow - dCol;

	while (true) {
		callback(row, col);

		if (row === row1 && col === col1) {
			break;
		}

		const err2 = err * 2;

		if (err2 > -dCol) {
			err -= dCol;
			row += stepRow;
		}

		if (err2 < dRow) {
			err += dRow;
			col += stepCol;
		}
	}
};

let isDrawing = false;
let lastCell = null;

canvas.addEventListener("mousedown", (event) => {
	const [row, col] = cellAtEvent(event);

	isDrawing = true;
	lastCell = [row, col];

	universe.toggle_cell(row, col);

	drawGrid();
	drawCells();
});

canvas.addEventListener("mousemove", (event) => {
	if (!isDrawing) {
		return;
	}

	const [row, col] = cellAtEvent(event);

	if (row === lastCell[0] && col === lastCell[1]) {
		return;
	}

	forEachCellOnLine(lastCell[0], lastCell[1], row, col, (r, c) => {
		universe.set_alive(r, c);
	});

	lastCell = [row, col];

	drawGrid();
	drawCells();
});

const stopDrawing = () => {
	isDrawing = false;
	lastCell = null;
};

window.addEventListener("mouseup", stopDrawing);
canvas.addEventListener("mouseleave", stopDrawing);

drawGrid();
drawCells();
play();
