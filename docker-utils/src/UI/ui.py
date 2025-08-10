import curses

class UI:
    def __init__(self):
        self.stdscr = None
        self.wins = []
        self.running = True
        self.selected = 0
        self.data = []
        self.headers = []
        self.num_windows = 0

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
        pass

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
        super().__init__()
        self.data = data
        self.headers = headers
        self.selected = 0
        self.num_windows = len(headers)

    def setup(self):
        pass

    def draw(self):
        max_y, max_x = self.stdscr.getmaxyx()
        self.stdscr.refresh()
        self.stdscr.addstr(0, 0, "Docker Containers", curses.A_BOLD)

        max_rows = max_y - 2

        self.resize_windows()

        for i, win in enumerate(self.wins):
            win.box()
            title = f" {self.headers[i]} "
            win_width = max_x // self.num_windows
            win.addstr(0, max(2, (win_width - len(title)) // 2), title)
            win.addstr(1, 2, self.headers[i][:win_width - 4])

            # Mostrar filas
            for row_i, item in enumerate(self.data[:max_rows]):
                line_y = 2 + row_i
                try:
                    text = self.row_formatter(item, i, win_width - 4)
                    if row_i == self.selected:
                        win.attron(curses.color_pair(1))
                        win.addstr(line_y, 2, text)
                        win.attroff(curses.color_pair(1))
                    else:
                        win.addstr(line_y, 2, text)
                except curses.error:
                    pass  # Ignorar errores por texto fuera de pantalla

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

    def row_formatter(self, item, win_idx, max_width):
        if win_idx == 0:
            txt = f"{item['name'][:max_width]}"
        elif win_idx == 1:
            txt = f"{item['id'][:max_width]}"
        elif win_idx == 2:
            txt = f"{item['image'][:max_width]}"
        elif win_idx == 3:
            txt = f"{item['status'][:max_width]}"
        else:
            txt = ""
        return txt.ljust(max_width)
