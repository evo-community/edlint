function off(rule) {
  if (Array.isArray(rule)) {
    return rule.map((r) => ({ ...r, severity: "off" }));
  }
  return { ...rule, severity: "off" };
}

function warn(rule) {
  if (Array.isArray(rule)) {
    return rule.map((r) => ({ ...r, severity: "warn" }));
  }
  return { ...rule, severity: "warn" };
}

const r = {
  requiredChildren: (abstractions) => ({
    name: "RequiredChildren",
    severity: "error",
    options: { abstractions },
  }),
  noUnabstractionFiles: () => ({
    name: "NoUnabstractionFiles",
    severity: "error",
    options: {},
  }),
  publicAbstraction: (name) => ({
    name: "PublicAbstraction",
    severity: "error",
    options: { name },
  }),
  restrictCrossImports: () => ({
    name: "RestrictCrossImports",
    severity: "error",
    options: {},
  }),
  dependenciesDirection: (order) => ({
    name: "DependenciesDirection",
    severity: "error",
    options: { order },
  }),
};

function normalizeFileTemplate(fileTemplate, fileTemplateUrl) {
  if (fileTemplateUrl) {
    return () => fs.readFileSync(fileTemplateUrl, "utf-8");
  }
  if (!fileTemplate) {
    return undefined;
  }
  if (typeof fileTemplate === "string") {
    return () => fileTemplate;
  }
  return fileTemplate;
}

function abstraction(optionsOrName, optionalConfig) {
  if (typeof optionsOrName === "string") {
    return {
      name: optionsOrName,
      children: optionalConfig?.children ?? {},
      rules: optionalConfig?.rules ?? [],
      fractal: optionalConfig?.fractal,
      fileTemplate: normalizeFileTemplate(
        optionalConfig?.fileTemplate,
        optionalConfig?.fileTemplateUrl
      ),
    };
  }
  return {
    name: optionsOrName.name,
    children: optionsOrName.children ?? {},
    rules: optionsOrName.rules ?? [],
    fractal: optionsOrName.fractal,
    fileTemplate: normalizeFileTemplate(
      optionsOrName.fileTemplate,
      optionsOrName?.fileTemplateUrl
    ),
  };
}

function defineConfig(config) {
  return config;
}

module.exports = {
  abstraction,
  defineConfig,
  off,
  warn,
  r,
};
