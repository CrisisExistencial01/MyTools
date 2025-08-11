import curses

class UI:
    def __init__(self, title):
        self.stdscr = None
        self.wins = []
        self.running = True
        self.selected = 0
        self.data = []
        self.headers = []
        self.num_windows = 0

        """
            header_box: dict
            {
                "title": str,
                "height": int,          by default 3
                "width": int,           by default full width
            }
        """
        self.header_box = {
            "title": title,
            "height": 3,
            "width": None,
            "enable": True
        }

    def start(self):
        """ Lanza la aplicación, maneja inicialización y ciclo principal """
        curses.wrapper(self._curses_main)

    def _curses_main(self, stdscr):
        self.stdscr = stdscr
        curses.curs_set(0)
        curses.start_color()
        curses.use_default_colors()
        self.init_colors()
        self.setup()

        self.prev_size = stdscr.getmaxyx()

        self.wins = [curses.newwin(1, 1, 0, 0) for _ in range(self.num_windows)]

        while self.running:
            self.handle_resize_if_needed()
            self.draw()
            key = self.stdscr.getch()
            self.handle_input(key)

    def init_colors(self):
        curses.init_pair(1, curses.COLOR_RED, curses.COLOR_BLACK)

    def setup(self):
        """
        Configura datos, ventanas, estado inicial.
        Debe sobreescribirse en subclases.
        """

        max_y, max_x = self.stdscr.getmaxyx()
        height = self.header_box["height"]
        self.header_win = curses.newwin(height, max_x, 0, 0)
        self.wins = [curses.newwin(max_y - height, max_x // self.num_windows, height, i * (max_x // self.num_windows))
            for i in range(self.num_windows)]

    def draw(self):
        """
        Dibuja la interfaz.
        Debe sobreescribirse en subclases.
        """
        pass

    def handle_input(self, key):
        """
        Procesa las teclas pulsadas.
        Por defecto salir con 'q'.
        """
        if key == ord('q'):
            self.running = False

    def handle_resize_if_needed(self):
        current_size = self.stdscr.getmaxyx()
        if current_size != self.prev_size:
            self.prev_size = current_size
            self.stdscr.erase()
            self.resize_windows()

    def resize_windows(self):
        """
        Ajusta tamaño y posiciones de las ventanas según tamaño actual.
        """
        max_y, max_x = self.stdscr.getmaxyx()
        win_width = max_x // self.num_windows
        win_height = max_y - 2

        for idx, w in enumerate(self.wins):
            w.resize(win_height, win_width)
            w.mvwin(0, idx * win_width)
            w.erase()

    def row_formatter(self, item, win_idx, max_width):
        """
        Fun. que toma un item y devuelve cadena formateada para cada ventana
        Por defecto vacía, debe redefinirse.
        """
        return ""


class DockerListUI(UI):
    def __init__(self, data, headers):
        super().__init__(title="Docker Containers List")
        self.data = data
        self.headers = headers
        self.selected = 0
        self.num_windows = len(headers)

    def resize_windows(self):
        max_y, max_x = self.stdscr.getmaxyx()
        self.header_win.resize(self.header_box["height"], max_x)
        self.header_win.mvwin(0, 0)

        win_width = max_x // self.num_windows
        win_height = max_y - self.header_box["height"]

        for idx, w in enumerate(self.wins):
            w.resize(win_height, win_width)
            w.mvwin(self.header_box["height"], idx * win_width)
            w.erase()

    def draw(self):
        max_y, max_x = self.stdscr.getmaxyx()
        self.stdscr.refresh()
        max_rows = max_y - 2 - self.header_box["height"]

        if self.header_win:
            self.header_win.box()
            header_text = self.header_box["title"]
            self.header_win.addstr(1, max(1, (max_x - len(header_text)) // 2), header_text, curses.A_BOLD)
            self.header_win.refresh()

            win_width = max_x // self.num_windows
            win_height = max_y - self.header_box["height"] - 1

            for idx, w in enumerate(self.wins):
                w.resize(win_height, win_width)
                w.mvwin(self.header_box["height"], idx * win_width)
                w.erase()

        for i, win in enumerate(self.wins):
            win.box()
            title = f" {self.headers[i]} "
            win_width = max_x // self.num_windows
            win.addstr(0, max(2, (win_width - len(title)) // 2), title)

            # Mostrar filas
            for row_i, item in enumerate(self.data[:max_rows]):
                line_y = 2 + row_i
                try:
                    text = self.row_formatter(item, i, win_width - 4, self.headers[i])
                    if row_i == self.selected:
                        win.attron(curses.color_pair(1))
                        win.addstr(line_y, 2, text)
                        win.attroff(curses.color_pair(1))
                    else:
                        win.addstr(line_y, 2, text)
                except curses.error:
                    pass

            win.refresh()

        self.stdscr.move(max_y - 1, 0)
        self.stdscr.clrtoeol()
        self.stdscr.refresh()
        self.stdscr.addstr(max_y - 1, 2, "Use ↑/↓ to navigate. Press 'q' to exit.")
        self.stdscr.refresh()

    def handle_input(self, key):
        data_size = len(self.data)
        if key == curses.KEY_UP:
            self.selected = (self.selected - 1) % data_size
        elif key == curses.KEY_DOWN:
            self.selected = (self.selected + 1) % data_size
        elif key == ord('q'):
            self.running = False

    def row_formatter(self, item, win_idx, max_width, header):
        txt = str(item.get(header, ""))[:max_width-1]

        return txt.ljust(max_width)
