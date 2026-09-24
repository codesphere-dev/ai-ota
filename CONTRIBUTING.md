# Contributing to ai-ota

First off, thank you for considering contributing to ai-ota! It's people like you that make this project better for everyone.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Workflow](#development-workflow)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Commit Message Guidelines](#commit-message-guidelines)

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to [remi.boivin@codesphere-dev.com](mailto:remi.boivin@codesphere-dev.com).

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When you create a bug report, include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples** (code snippets, screenshots, etc.)
- **Describe the behavior you observed and what you expected**
- **Include your environment details** (OS, Node version, Docker version, etc.)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion:

- **Use a clear and descriptive title**
- **Provide a detailed description of the suggested enhancement**
- **Explain why this enhancement would be useful**
- **Include mockups or examples if applicable**

### Contributing Code

Ready to contribute code? Great! Follow our [Git Workflow](GIT.md) and [Development Guide](DEV.md).

## Development Workflow

### 1. Fork the Repository

Fork the `codesphere-dev/ai-ota` repository to your GitHub account.

### 2. Clone Your Fork

```bash
git clone https://github.com/YOUR_USERNAME/ai-ota.git
cd ai-ota
```

### 3. Add Upstream Remote

```bash
git remote add upstream https://github.com/codesphere-dev/ai-ota.git
```

### 4. Set Up Development Environment

Follow the instructions in [DEV.md](DEV.md) to set up your local development environment.

### 5. Create a Branch

Create a branch following our naming conventions (see [GIT.md](GIT.md)):

```bash
git checkout -b feat/your-feature-name
# or
git checkout -b bug/your-bug-fix
# or
git checkout -b hotfix/critical-issue
```

### 6. Make Your Changes

- Write clean, maintainable code
- Follow our [coding standards](#coding-standards)
- Add tests for new features
- Ensure all tests pass
- Update documentation as needed

### 7. Commit Your Changes

Follow our [commit message guidelines](#commit-message-guidelines):

```bash
git add .
git commit -m "feat: add amazing new feature"
```

### 8. Keep Your Fork Updated

```bash
git fetch upstream
git rebase upstream/develop
```

### 9. Push to Your Fork

```bash
git push origin feat/your-feature-name
```

### 10. Create a Pull Request

Go to the original repository and create a Pull Request from your fork to the `develop` branch. See [Pull Request Process](#pull-request-process) below.

## Pull Request Process

### Before Submitting

- [ ] Ensure your code follows our coding standards
- [ ] All tests pass (`npm test` in both client and server)
- [ ] Update documentation if needed
- [ ] Add tests for new features
- [ ] Rebase on latest `develop` branch
- [ ] Ensure no merge conflicts

### PR Description

Your PR description should include:

1. **Summary**: Brief description of changes
2. **Motivation**: Why this change is needed
3. **Changes**: List of key changes
4. **Testing**: How to test the changes
5. **Screenshots**: If UI changes are involved
6. **Related Issues**: Link to related issues (e.g., "Closes #123")

### Review Process

1. At least one maintainer must approve your PR
2. All CI checks must pass
3. Code must be up-to-date with the `develop` branch
4. Address all review comments
5. Once approved, a maintainer will merge your PR

## Coding Standards

### General

- Use meaningful variable and function names
- Write self-documenting code
- Add comments for complex logic
- Keep functions small and focused
- Follow DRY (Don't Repeat Yourself) principle

### Testing

- Write unit tests for all business logic
- Write integration tests for API endpoints
- Aim for high test coverage (>80%)
- Use descriptive test names
- Follow AAA pattern (Arrange, Act, Assert)

### File Naming

- React components: `PascalCase.jsx` or `PascalCase.tsx`
- Utilities: `camelCase.js`
- Tests: `*.test.js` or `*.spec.js`
- Hooks: `useCamelCase.js`

## Commit Message Guidelines

We use a structured commit message format that provides context and reasoning for changes. For detailed guidelines, see [GIT.md](GIT.md).

### Format

```
<type>(<scope>): <subject> [<code>]

WHY:
<explanation of why this change is needed>

WHAT:
- <list of changes>
- <another change>

```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that don't affect code meaning (formatting, etc.)
- **refactor**: Code change that neither fixes a bug nor adds a feature
- **perf**: Performance improvement
- **test**: Adding or updating tests
- **chore**: Changes to build process or auxiliary tools
- **ci**: Changes to CI configuration files and scripts

### Example

```bash
chore(docs): add initial project documentation and git setup [WRC1]

WHY:
Provide essential baseline documentation and project configuration files to guide 
contributors, ensure security awareness, define code of conduct, and establish git 
workflow conventions.

WHAT:
- Added README.md with project description, badges, and template sections
- Added CONTRIBUTING.md with guidelines for contributions and workflow
- Added SECURITY.md with security reporting instructions
- Added CODE_OF_CONDUCT.md with community behavior standards
- Added .gitignore and basic git configuration files
```

### Rules

- Use present tense ("add feature" not "added feature")
- Use imperative mood ("move cursor to..." not "moves cursor to...")
- Don't capitalize first letter
- No period at the end
- Keep subject line under 72 characters
- Include WHY, WHAT, Web Impact, and Failure Mode sections
- Wrap body text at 72 characters
- See [GIT.md](GIT.md) for more examples and details

## Questions?

If you have questions, feel free to:

- Open an issue for discussion
- Check our [Development Guide](DEV.md)
- Check our [Git Workflow](GIT.md)
- Contact the maintainers

Thank you for contributing! 🎉
