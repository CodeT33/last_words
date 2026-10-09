

---

## 1. Concept

**Last Words** is a fast-paced, 2D cooperative typing minigame for one or two players. Players attempt to preserve as
much written material as possible before a round timer expires.

The game is presented as stacks of loose paper leaves. Each stack contains multiple pages, and players choose which
stack to work through by typing the visible first word on its top page. Once a page is revealed, the player must decide
whether to discard it or transcribe it before returning to the stack selection screen.

The core tension comes from balancing typing speed, accuracy, page difficulty, score value, and the time remaining.

## 2. Round Structure

At the start of a round, players choose a duration:

- **1 minute** — short, intense arcade round

- **5 minutes** — standard round

- **10 minutes** — longer session for high scores

One or two players participate in the same round and share the round timer and overall objective.

### Main gameplay loop

**Choose a stack**

- Several paper stacks are displayed on the screen.

- Each stack shows the first word of its top page and an indicator of its difficulty or reward.

- Players select a stack by typing its displayed first word.

**Reveal the top page**

- The selected page is removed from the stack and shown in the typing view.

- Its full text, difficulty, and potential score value are displayed.


**Decide what to do**

- **Transcribe:** Type the page as accurately as possible.

- **Discard:** Abandon the page to return to stack selection more quickly.

- Discarded pages are permanently lost and cannot be recovered later.


**Resolve the page**

- When the transcription is completed successfully, the page is saved and its reward is added to the team's score.

- If the page is discarded, it is permanently lost and gives no reward.

- After either action, return to stack selection.


**Repeat until time runs out**

- Players continue selecting, transcribing, and discarding pages until the shared timer reaches zero.


**Show the results**

- Calculate how much text was saved.

- Display the final score and round statistics.

## 3. Paper Stacks and Pages

Each stack represents a category of pages with a different risk/reward profile.

A stack should communicate:

- **Top word:** The word players must type to select the stack.

- **Difficulty:** An at-a-glance indicator of the pages inside.

- **Potential reward:** The approximate score or value of its pages.

- **Remaining pages:** How many pages are left in the stack, if this information is visible to players.

Possible stack types:

- **Easy:** Shorter, simpler pages with lower rewards.

- **Medium:** Moderate text length and difficulty with balanced rewards.

- **Hard:** Longer or more challenging pages with higher rewards.

The exact relationship between difficulty and reward should be tuned during playtesting. Hard pages should offer a
meaningful advantage without always being mathematically the best choice.

## 4. Typing Mechanics

The typing system is the central mechanic and should feel responsive and immediate.

Required behavior:

- Capture keyboard input while the typing interface is active.

- Compare typed characters against the target text.

- Clearly distinguish correct and incorrect characters.

- Show transcription progress.

- Detect when the full page has been completed.

- Prevent input from leaking into stack selection or other UI states.

- Support punctuation, spaces, capitalization, and line breaks according to the chosen typing rules.

### Typing rules to decide

- Are mistakes allowed, or must players correct every error before continuing?

- Does incorrect input reduce the score, add a time penalty, break a combo, or some combination?

- Is a page only saved after a perfect transcription, or can partially correct text earn a reduced reward?

- Does the game require exact capitalization and punctuation?

- Can players hold backspace and correct mistakes freely?

For a fast arcade feel, keep feedback immediate and avoid lengthy animations between pages.

## 5. Scoring and Saved Text

The main objective is to save as much text as possible before time expires.

Potential score factors:

- **Characters or words saved:** The primary measure of progress.

- **Page difficulty:** Harder pages can award more points.

- **Accuracy:** Reward precise transcription.

- **Completion:** Award a bonus for fully completed pages.

- **Streaks or combos:** Reward consecutive pages completed without mistakes, if desired.

Decide whether the main result is measured in saved words, saved characters, points, or a combination. A clear primary
metric will make the game easier to understand.

A saved page should be recorded as a completed page, not just a score increment, so the results screen can show the
actual amount of text preserved.

## 6. Timer and Round States

The round timer is shared by both players.

Required states:

- **Ready / Start:** Briefly introduce the round and begin the timer.

- **Stack Selection:** Players choose a stack by typing its visible top word.

- **Page Transcription:** A selected page is displayed for typing.

- **Page Resolution:** Save or discard the page and return to selection.

- **Round Over:** Stop accepting gameplay input and calculate results.

- **Results:** Display the final score and statistics.

The timer should continue running during stack selection, transcription, and discard decisions to maintain urgency.

When the timer reaches zero, the round ends immediately. Decide whether an unfinished page is lost or whether completion
is allowed if the final keystroke occurs at the exact time limit. Apply the rule consistently.

## 7. One-Player and Two-Player Modes

### Single-player

- One player manages stack selection and transcription.

- The player is responsible for all decisions and typing.

### Two-player co-op

- Both players contribute to the same round objective.

- The timer and team score are shared.

- Saved pages from either player contribute to the final result.

The implementation must define how players interact with pages:

- **Shared focus:** Both players can help type the currently selected page.

- **Independent work:** Each player has a separate active page while contributing to the same shared score.

- **Alternating turns:** Players take turns selecting and transcribing pages.

For the first version, choose one interaction model and keep it consistent. Independent work may create a more active
co-op experience, but it also requires more UI space and more complex input/state handling.

## 8. User Interface

The screen should prioritize readability and speed.

Required UI elements:

- Round timer

- Team score and saved-text total

- Multiple stack cards or stack illustrations

- Each stack's visible top word

- Difficulty and reward indicator

- Remaining page count, if applicable

- Current page text

- Typing progress and character-level feedback

- Clear Transcribe and Discard actions

- End-of-round results screen

Use strong visual hierarchy so players can understand their options at a glance. Keep transitions short and avoid
unnecessary confirmation dialogs.

## 9. Content and Data Model

Pages should be stored as data rather than hard-coded into gameplay logic.

A page should contain, at minimum:

- Unique page ID

- Full text

- Difficulty category or value

- Score value or reward parameters

- Word/character count, either stored or calculated

- Optional metadata for visual presentation

A stack should contain:

- Unique stack ID

- Stack category or difficulty

- Ordered list of page IDs or page data

- Current position or remaining pages

A round should track:

- Selected duration and remaining time

- Current game state

- Saved pages

- Discarded pages

- Team score

- Per-player statistics, if needed

Keep page content, gameplay rules, and rendering separate so that new text collections and balancing changes can be
added easily.

## 10. Results Screen

At the end of the round, display:

- Total score

- Total words or characters saved

- Number of pages saved

- Number of pages discarded

- Typing accuracy

- Average or peak words per minute

- Optional personal/team record

The results should reinforce the theme: show the rescued pages as a preserved collection or manuscript, rather than
presenting only abstract numbers.

## 11. Audio and Visual Feedback

Recommended feedback:

- Distinct sound for selecting a stack

- Subtle typewriter sounds while typing

- Immediate visual feedback for correct and incorrect characters

- A satisfying completion sound when a page is saved

- A discard sound and visual effect when a page is lost

- Timer warnings near the end of the round

- A clear round-ending effect

The visual style can draw on experimental literary layouts: paper textures, monochrome typography, marginal notes,
footnotes, red editorial marks, and subtly unsettling page arrangements. Keep the actual text legible during fast
gameplay.

## 12. Implementation Checklist

### Core gameplay

- [ ] Round duration selection (1, 5, or 10 minutes)

- [ ] Round timer and end condition

- [ ] Stack selection by typing the displayed top word

- [ ] Stack contents and page progression

- [ ] Page reveal view

- [ ] Transcribe action

- [ ] Discard action

- [ ] Permanent removal of discarded pages

- [ ] Return to stack selection after resolving a page

- [ ] Successful page completion and saving

### Typing system

- [ ] Keyboard input capture

- [ ] Character-by-character text matching

- [ ] Correct/incorrect character feedback

- [ ] Progress display

- [ ] Error correction rules

- [ ] Completion detection

- [ ] Input handling based on current game state

### Scoring and results

- [ ] Page reward values

- [ ] Saved-text tracking

- [ ] Score calculation

- [ ] Accuracy and WPM calculation

- [ ] Results screen

- [ ] Optional combo/streak system

### Content and balancing

- [ ] Page data format

- [ ] Easy, medium, and hard stacks

- [ ] Text selection and stack population

- [ ] Balance page length, difficulty, and reward

- [ ] Ensure the top word clearly identifies each available stack

- [ ] Test whether stack selection takes an appropriate amount of time

### UI and presentation

- [ ] Main menu / round setup

- [ ] Stack selection UI

- [ ] Page transcription UI

- [ ] Timer and score display

- [ ] Transcribe and Discard controls

- [ ] Short transitions and animations

- [ ] Audio and visual feedback

- [ ] Results screen

### Co-op

- [ ] Decide the two-player interaction model

- [ ] Shared timer and team score

- [ ] Track player contributions

- [ ] Handle simultaneous input and page selection

- [ ] Test readability and usability with two players

### Testing

- [ ] Verify that the timer cannot be bypassed

- [ ] Verify discarded pages cannot be recovered

- [ ] Verify completed pages are counted exactly once

- [ ] Test edge cases when time expires during transcription

- [ ] Playtest 1-, 5-, and 10-minute rounds

- [ ] Tune rewards so hard stacks are tempting but not always optimal

## 13. Suggested Development Order

1. **Typing prototype:** Display one page and implement fast, accurate transcription.

2. **Round loop:** Add the timer, stack selection, page completion, and discard flow.

3. **Scoring:** Track saved text and show results.

4. **Content variation:** Add multiple stacks with different lengths, difficulties, and rewards.

5. **Presentation:** Add paper visuals, typography, sounds, and fast transitions.

6. **Co-op:** Implement and test the chosen two-player model.

7. **Balancing:** Tune round durations, rewards, typing rules, and decision speed.

## 14. Technical Direction

The current plan is to use **raylib** rather than Bevy. Keep the game architecture small and explicit:

- Separate game-state logic from rendering.

- Keep typing input and text matching in a dedicated module.

- Represent pages and stacks as data.

- Centralize score and timer updates.

- Use a clear state machine for the round flow.

- Keep the first version focused on one screen and one core loop before adding networking or elaborate visual effects.

The goal of the first playable prototype is simple: a player can choose a stack, type a page, save or discard it, repeat
until the timer expires, and see how much text was rescued.