# Asynchroniczne bingo

Gra jest przeznaczona dla kilku 4-osobowych zespołów.

## Przygotowanie

1. W każdym zespole przypisujemy osobom indeksy od 1 do 4.
Jeśli zespół ma mniej niż 4 osoby, to i tak rozdzielamy wszystkie indeksy (niektórzy będą mieć więcej niż jeden).
2. Rozdzielamy losowe 4 asynchroniczne funkcje (`task_a`, `task_b`, `task_c`, `task_d`) pomiędzy osoby w zespole.
Ważne jest, żeby przypisanie było losowe względem indeksów (np. osoba z indeksem 1 nie powinna mieć zawsze `task_a`).
3. Każda osoba w zespole potrzebuje brudnopisu, gdzie będzie mogła **ręcznie** prowadzić proste obliczenia na liczbach rzeczywistych.
4. Na każdy zespół przypada dokładnie jeden (współdzielony) długopis.
5. Każdy zespół ma kilka małych karteczek (potrzebnych jest 8 na jedną rozgrywkę), na których będą zapisywane częściowe wyniki i przesyłane pomiędzy taskami (osobami).

## Przebieg rozgrywki

### Zachowanie w zespole

1. Każda osoba może być w jednym z trzech stanów:
    - **czeka** na wyniki od innych osób lub input z zewnętrznego źródła (tablica),
    - **oblicza** wyniki na podstawie wyników od innych osób,
    - **gotowa** (funkcja doszła do końca wykonania).
2. Żeby prowadzić obliczenie **należy** użyć długopisu.
Jeśli ktoś inny w zespole prowadzi obliczenie, to należy poczekać na swoją kolej.
3. Po zakończeniu obliczeń należy odłożyć długopis.
4. Wysyłając wiadomość do innej osoby należy użyć karteczki i położyć ją przed odbiorcą.
5. Jeśli ktoś dojdzie do instrukcji `println!("[B] BINGO: {result}")` należy niezwłocznie to zgłosić.
Jeśli wynik (`result`) jest poprawny, to zespół wygrywa grę.

### Przydzielanie kontroli

1. Prowadzący w sposób ciągły losuje indeksy od 1 do 4 (i je ogłasza).
2. Osoba z wywołanym indeksem ma możliwość podjęcia akcji (kontynuacji wykonywania swojego taska).
W szczególności, jeśli ma wszystkie dane potrzebne do obliczeń **oraz** długopis jest wolny, może zacząć obliczenia.
W każdym innym razie musi poczekać na kolejne wywołanie.

### Zewnętrzne źródła danych

1. Dwa taski oczekują na dane z zewnętrznego źródła (tablicy) - te zostaną opublikowane przez prowadzącego w pewnym momencie gry.

# Inne warianty

- 2 długopisy (jako dwa procesory)
- ograniczone kanały
- kanały rendez vous
- ...
