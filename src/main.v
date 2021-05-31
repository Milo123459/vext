import term.ui as tui

struct App {
mut:
	tui &tui.Context = 0
}

fn event(e &tui.Event, x voidptr) {
	mut app := &App(x)
	app.tui.set_cursor_position(0, 0)

	app.tui.flush()

	if e.typ == .key_down && e.code == .escape {
		exit(0)
	}
}

fn main() {
	mut app := &App{}
	app.tui = tui.init(
		user_data: app
		event_fn: event
		window_title: 'Vext'
		hide_cursor: false
		capture_events: true
		frame_rate: 60
		use_alternate_buffer: false
	)
	app.tui.clear()
	
	app.tui.run() ?
}