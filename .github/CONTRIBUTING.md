# Contributing 🚀

Thank you for considering contributing to **InkCheck**! Your contributions help improve the tool and make it more useful for everyone. Below is a guide to help you get started with the contribution process.

## How to Contribute

### 1. Fork the Repository

To contribute, fork the repository to your GitHub account. This allows you to make changes freely without affecting the main project until you're ready to submit them.

### 2. Create a New Branch 🌱

Create a new branch for your changes to keep your work isolated and make it easier to submit a pull request later. You can create a new branch using the following command:

```bash
git checkout -b your-feature-branch
```

### 3. Adding Printer OIDs 🖨️

In the `src/data` directory, you'll find JSON files containing **SNMP OIDs** for different printer brands. These OIDs are crucial for retrieving supply data from printers.

If you'd like to contribute additional printer models, please add the OIDs for those models to the appropriate JSON file in the src/data directory. Contributions of new printer OIDs are highly appreciated, as they help expand the compatibility to support more printer models.

**Note:** The structure of the OID fields is fixed. Whether or not a printer has a specific feature (like toner, drum, fuser, etc.), the field must exist in the JSON file. If the printer does not support a particular feature, the respective fields should be left blank.

For example, in the case of a **Xerox** printer:

```json
{
    "Xerox AltaLink C8030 Multifunction Printer": {
        "toner": {
            "black": { "level": "1.3.6.1.2.1.43.11.1.1.9.1.1", "max_level": "1.3.6.1.2.1.43.11.1.1.8.1.1" },
            "cyan": { "level": "1.3.6.1.2.1.43.11.1.1.9.1.2", "max_level": "1.3.6.1.2.1.43.11.1.1.8.1.2" },
            "magenta": { "level": "1.3.6.1.2.1.43.11.1.1.9.1.3", "max_level": "1.3.6.1.2.1.43.11.1.1.8.1.3" },
            "yellow": { "level": "1.3.6.1.2.1.43.11.1.1.9.1.4", "max_level": "1.3.6.1.2.1.43.11.1.1.8.1.4" }
        },
        "drum": { "black": { "level": "1.3.6.1.2.1.43.11.1.1.9.1.5", "max_level": "1.3.6.1.2.1.43.11.1.1.8.1.5" }, "cyan": { "level": "1.3.6.1.2.1.43.11.1.1.9.1.6", "max_level": "1.3.6.1.2.1.43.11.1.1.8.1.6" }, "magenta": { "level": "1.3.6.1.2.1.43.11.1.1.9.1.7", "max_level": "1.3.6.1.2.1.43.11.1.1.8.1.7" }, "yellow": { "level": "1.3.6.1.2.1.43.11.1.1.9.1.8", "max_level": "1.3.6.1.2.1.43.11.1.1.8.1.8" } },
        "fuser": { "level": "1.3.6.1.2.1.43.11.1.1.9.1.9", "max_level": "1.3.6.1.2.1.43.11.1.1.8.1.9" },
        "reservoir": { "level": "1.3.6.1.2.1.43.11.1.1.9.1.10", "max_level": "1.3.6.1.2.1.43.11.1.1.8.1.10" }
    }
}
```

For a printer like the **B431**, the fields must still be present, even if some values are empty:

```json
{
    "B431": {
        "toner": {
            "black": { "level": "1.3.6.1.2.1.43.11.1.1.9.1.1", "max_level": "1.3.6.1.2.1.43.11.1.1.8.1.1" },
            "cyan": { "level": "", "max_level": "" },
            "magenta": { "level": "", "max_level": "" },
            "yellow": { "level": "", "max_level": "" }
        },
        "drum": {
            "black": { "level": "1.3.6.1.2.1.43.11.1.1.9.1.2", "max_level": "1.3.6.1.2.1.43.11.1.1.8.1.2" },
            "cyan": { "level": "", "max_level": "" },
            "magenta": { "level": "", "max_level": "" },
            "yellow": { "level": "", "max_level": "" }
        },
        "fuser": { "level": "", "max_level": "" },
        "reservoir": { "level": "", "max_level": "" }
    }
}

```

### 4. Linting and Code Quality 🧹

Before submitting your changes, please make sure that your code follows the project's coding standards. We recommend using the `just` command to perform linting and check your code with **Clippy**. To do this, run:


```elixir
just lint
```

This will ensure your code is free of common issues and adheres to the project's style guidelines.

### 5. Writing Tests 🧪

If you are adding new functionality or modifying existing code, please write tests to ensure that everything works as expected. The tests help maintain the general reliability and stability.

### 6. Submit a Pull Request 📤

Once you've made your changes, it's time to submit a pull request. When doing so, make sure to:

- Provide a clear and descriptive title for your pull request.
- Include a summary of the changes you've made and why they are necessary.
- Reference any issues that your pull request addresses (if applicable).
- Make sure your pull request is targeting the main branch.

We truly appreciate your contributions! Every improvement, big or small, helps make a better tool for everyone. If you have any questions or need help, feel free to ask. We're here to help!

Happy coding! 🎉
