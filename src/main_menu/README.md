# Main Menu

<table>
  <tr>
    <td>
      <img src="https://avatars.githubusercontent.com/u/72411904?v=4" alt="John Klucinec" width="100">
    </td>
    <td>
      <strong>Author:</strong>
      <a href="https://github.com/johnklucinec">John Klucinec  </a>
    </td>
  </tr>
</table>

---

## Directory Structure

- 📁 **main_menu**
    - 📁 **systems**
        - 📄 **interaction.rs** - *Button interaction handlers*
        - 📄 **layout.rs** - *UI layout and spawning logic*
        - 📄 **mod.rs** - *Systems module exports*
    - 📄 **components.rs** - *UI component markers*
    - 📄 **mod.rs** - *Plugin definition and exports*
    - 📄 **styles.rs** - *UI styling constants and functions*

## Main Menu Image

![image](https://github.com/user-attachments/assets/ebcff3d8-275c-45bf-aa23-e8345842359d)

---


## Components

The menu consists of several key components:


| Component | Description |
| :-- | :-- |
| `MainMenu` | Root container for the entire menu UI |
| `PlayButton` | Button that transitions to the game state |
| `QuitButton` | Button that exits the application |
| `DisabledButton` | Template for future menu options (currently inactive) |

*Part of the Bevy Simulator project.*
