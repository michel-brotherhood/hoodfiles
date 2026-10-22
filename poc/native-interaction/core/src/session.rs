//! Máquina de estados da sessão "arrastar + atalho + soltar". Pura: recebe eventos,
//! devolve ações. A camada de plataforma executa as ações e mede o que realmente ocorre.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Idle,
    /// Atalho pressionado, overlay visível, aguardando arquivos.
    OverlayVisible,
    /// Arraste externo entrou no overlay; caminhos recebidos (contagem apenas).
    DragOver {
        count: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    ShortcutPressed,
    ShortcutReleased,
    DragEntered { count: usize },
    DragLeft,
    Dropped { count: usize },
    EscapePressed,
    Timeout,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    ShowOverlay,
    HideOverlay,
    /// Registrar caminhos recebidos como entradas autorizadas.
    AcceptDrop,
    Log(&'static str),
}

/// Como o overlay se comporta com o atalho: manter pressionado ou alternar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutMode {
    Toggle,
    HoldToShow,
}

#[derive(Debug)]
pub struct Session {
    pub state: State,
    pub mode: ShortcutMode,
}

impl Session {
    pub fn new(mode: ShortcutMode) -> Self {
        Self {
            state: State::Idle,
            mode,
        }
    }

    pub fn handle(&mut self, ev: Event) -> Vec<Action> {
        use Action::*;
        use State::*;
        let (next, actions) = match (&self.state, ev) {
            (Idle, Event::ShortcutPressed) => (OverlayVisible, vec![ShowOverlay]),
            (OverlayVisible | DragOver { .. }, Event::ShortcutPressed)
                if self.mode == ShortcutMode::Toggle =>
            {
                (Idle, vec![HideOverlay, Log("cancelado: atalho repetido")])
            }
            (OverlayVisible, Event::ShortcutReleased) if self.mode == ShortcutMode::HoldToShow => (
                Idle,
                vec![HideOverlay, Log("cancelado: atalho solto sem drop")],
            ),
            // Em HoldToShow, soltar o atalho durante o arraste não deve esconder o alvo.
            (DragOver { .. }, Event::ShortcutReleased) => (self.state.clone(), vec![]),
            (OverlayVisible, Event::DragEntered { count }) => (DragOver { count }, vec![]),
            (DragOver { .. }, Event::DragLeft) => (OverlayVisible, vec![]),
            (OverlayVisible | DragOver { .. }, Event::Dropped { .. }) => {
                (Idle, vec![AcceptDrop, HideOverlay])
            }
            (OverlayVisible | DragOver { .. }, Event::EscapePressed) => {
                (Idle, vec![HideOverlay, Log("cancelado: Escape")])
            }
            (OverlayVisible | DragOver { .. }, Event::Timeout) => {
                (Idle, vec![HideOverlay, Log("cancelado: timeout")])
            }
            // Eventos fora de contexto são ignorados (ex.: drop sem overlay, Escape ocioso).
            (s, _) => (s.clone(), vec![]),
        };
        self.state = next;
        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Action::*;

    #[test]
    fn happy_path_toggle() {
        let mut s = Session::new(ShortcutMode::Toggle);
        assert_eq!(s.handle(Event::ShortcutPressed), vec![ShowOverlay]);
        assert!(s.handle(Event::DragEntered { count: 3 }).is_empty());
        assert_eq!(s.state, State::DragOver { count: 3 });
        assert_eq!(
            s.handle(Event::Dropped { count: 3 }),
            vec![AcceptDrop, HideOverlay]
        );
        assert_eq!(s.state, State::Idle);
    }

    #[test]
    fn escape_cancels_from_both_visible_states_and_is_noop_idle() {
        let mut s = Session::new(ShortcutMode::Toggle);
        assert!(s.handle(Event::EscapePressed).is_empty());
        s.handle(Event::ShortcutPressed);
        assert!(s.handle(Event::EscapePressed).contains(&HideOverlay));
        s.handle(Event::ShortcutPressed);
        s.handle(Event::DragEntered { count: 1 });
        assert!(s.handle(Event::EscapePressed).contains(&HideOverlay));
        assert_eq!(s.state, State::Idle);
    }

    #[test]
    fn drop_without_overlay_is_ignored() {
        let mut s = Session::new(ShortcutMode::Toggle);
        assert!(s.handle(Event::Dropped { count: 1 }).is_empty());
        assert_eq!(s.state, State::Idle);
    }

    #[test]
    fn hold_mode_release_before_drag_hides_but_release_during_drag_keeps_target() {
        let mut s = Session::new(ShortcutMode::HoldToShow);
        s.handle(Event::ShortcutPressed);
        assert!(s.handle(Event::ShortcutReleased).contains(&HideOverlay));
        s.handle(Event::ShortcutPressed);
        s.handle(Event::DragEntered { count: 2 });
        assert!(s.handle(Event::ShortcutReleased).is_empty());
        assert_eq!(s.state, State::DragOver { count: 2 });
    }

    #[test]
    fn drag_leave_returns_to_visible_and_toggle_repress_cancels() {
        let mut s = Session::new(ShortcutMode::Toggle);
        s.handle(Event::ShortcutPressed);
        s.handle(Event::DragEntered { count: 1 });
        s.handle(Event::DragLeft);
        assert_eq!(s.state, State::OverlayVisible);
        assert!(s.handle(Event::ShortcutPressed).contains(&HideOverlay));
    }

    #[test]
    fn timeout_cancels() {
        let mut s = Session::new(ShortcutMode::Toggle);
        s.handle(Event::ShortcutPressed);
        assert!(s.handle(Event::Timeout).contains(&HideOverlay));
    }
}
