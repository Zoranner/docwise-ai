export type WorkspaceTab = "blueprint" | "task";

export type ProjectStatus = "planning" | "active" | "blocked" | "done";

export type BlueprintStatus = "draft" | "approved" | "active" | "revised";

export type BlueprintItemStatus = "planned" | "active" | "done";

export type TaskStatus =
  | "backlog"
  | "running"
  | "waiting_checkpoint"
  | "blocked"
  | "done";

export type Project = {
  id: string;
  name: string;
  workspaceId: string;
  workspacePath: string;
  readablePaths: string[];
  stage: string;
  status: ProjectStatus;
  progress: number;
  summary: string;
  updatedAt: string;
  archivedAt?: string | null;
};

export type Blueprint = {
  id: string;
  projectId: string;
  title: string;
  status: BlueprintStatus;
  goal: string;
  audience: string;
  constraints: string[];
};

export type BlueprintItem = {
  id: string;
  projectId: string;
  blueprintId: string;
  code: string;
  filePath: string;
  title: string;
  goal: string;
  summary: string;
  briefingMarkdown: string;
  status: BlueprintItemStatus;
  progress: number;
};

export type TaskNode = {
  id: string;
  projectId: string;
  blueprintItemId: string;
  parentId: string | null;
  title: string;
  summary: string;
  briefingMarkdown: string;
  owner: string;
  status: TaskStatus;
  progress: number;
};

export type ProjectEvent = {
  id: string;
  projectId: string;
  kind: "blueprint" | "task" | "decision" | "output" | "project";
  title: string;
  detail: string;
  at: string;
  tone: "primary" | "neutral" | "warning" | "success";
};

export type ProjectConversation = {
  id: string;
  projectId: string;
  title: string;
  updatedAt: string;
};

export type ProjectAgentKind = "planner" | "executor" | "reviewer" | "coordinator";

export type ProjectAgentActivity = {
  kind: "thought" | "tool";
  label: string;
  durationLabel: string;
  meta?: string | null;
  body?: string | null;
  relatedFiles?: string[];
};

export type ProjectMessage = {
  id: string;
  projectId: string;
  role: "user" | "agent";
  content: string;
  at: string;
  agentId?: string | null;
  agentName?: string | null;
  agentKind?: ProjectAgentKind | null;
  handledDuration?: string | null;
  activities?: ProjectAgentActivity[];
};

export const mockProjects: Project[] = [
  {
    id: "p-alpha",
    name: "客户 A 交付包",
    workspaceId: "ws-alpha",
    workspacePath: "E:/work/client-a-delivery",
    readablePaths: ["E:/work/shared-delivery-assets", "E:/work/common-templates"],
    stage: "蓝图批准后执行中",
    status: "active",
    progress: 68,
    summary: "正在把交付目录、部署说明和 FAQ 收拢成客户可读的最终交付包。",
    updatedAt: "18:22",
    archivedAt: null,
  },
  {
    id: "p-beta",
    name: "内部 SOP 重写",
    workspaceId: "ws-beta",
    workspacePath: "E:/work/internal-sop",
    readablePaths: ["E:/work/internal-policies"],
    stage: "等待责任边界裁决",
    status: "blocked",
    progress: 41,
    summary: "蓝图骨架已成形，但审批责任人与发布顺序仍需人工拍板。",
    updatedAt: "17:48",
    archivedAt: null,
  },
  {
    id: "p-gamma",
    name: "实验性知识库整理",
    workspaceId: "ws-gamma",
    workspacePath: "E:/work/lab-knowledge-base",
    readablePaths: ["E:/work/lab-notes", "E:/work/archive-kb"],
    stage: "首轮产出完成",
    status: "done",
    progress: 83,
    summary: "已完成知识域归档与索引重建，等待下一轮扩写和校对指令。",
    updatedAt: "昨天 18:40",
    archivedAt: null,
  },
];

export const mockBlueprints: Blueprint[] = [
  {
    id: "bp-alpha-1",
    projectId: "p-alpha",
    title: "客户交付文档主蓝图",
    status: "approved",
    goal: "输出一组客户可直接接收的交付文档，覆盖背景、部署、FAQ 与验收入口。",
    audience: "客户项目负责人、实施负责人、运维接口人",
    constraints: ["不暴露内部校对记录", "部署文档必须包含环境变量说明"],
  },
  {
    id: "bp-beta-1",
    projectId: "p-beta",
    title: "SOP 职责重组蓝图",
    status: "revised",
    goal: "按职责链而不是系统模块重组 SOP，形成可执行的内部流程说明。",
    audience: "运营经理、支持工程师、审批角色",
    constraints: ["先明确审批责任人", "暂不处理历史遗留流程外例"],
  },
  {
    id: "bp-gamma-1",
    projectId: "p-gamma",
    title: "知识库归档蓝图",
    status: "active",
    goal: "重建主题域索引与更新节奏，形成可持续维护的知识库目录。",
    audience: "内部知识维护者、技术支持、研发值班人员",
    constraints: ["暂不清理过期旧文档", "优先完成索引与导航层"],
  },
];

export const mockBlueprintItems: BlueprintItem[] = [
  {
    id: "bi-alpha-1",
    projectId: "p-alpha",
    blueprintId: "bp-alpha-1",
    code: "A01",
    filePath: "docs/delivery-outline.md",
    title: "交付包范围与目录",
    goal: "定义客户最终会收到哪些文档与附件，以及整体目录结构。",
    summary: "当前已完成范围草案，正在确认附件边界。",
    briefingMarkdown: `# 交付文档编写要求

## 文档目标
明确客户最终会收到哪些文档、附件和补充说明，先把交付边界说清楚，再展开目录结构。

## 面向对象
- 客户项目负责人
- 实施负责人
- 运维接口人

## 本文必须覆盖
- 最终交付包包含哪些主文档
- 附件、脚本、示例配置分别放在哪一层目录
- 哪些内部材料不能进入客户交付包
- 验收入口和阅读顺序如何安排

## 编写提示
- 先给总目录，再说明每个文档的用途
- 对客户可见内容使用外部口径，避免内部术语
- 如果某项内容仍未确认，用“待确认”标注，不要自行补全`,
    status: "active",
    progress: 82,
  },
  {
    id: "bi-alpha-2",
    projectId: "p-alpha",
    blueprintId: "bp-alpha-1",
    code: "A02",
    filePath: "docs/deployment.md",
    title: "部署与环境变量说明",
    goal: "提供面向客户实施方的部署步骤、环境变量和回滚说明。",
    summary: "部署章节已起草，环境变量仍在收紧。",
    briefingMarkdown: `# 部署说明编写要求

## 写作重点
这份文档不是给研发看的，而是给实施方和运维方按步骤执行的。

## 需要组织的内容
- 部署前准备
- 依赖环境与版本要求
- 环境变量与默认值说明
- 启动、验证、回滚

## 特别注意
- 环境变量要解释含义和风险，不要只列键名
- 所有命令都要给出执行位置或前置条件
- 回滚章节必须和部署步骤一一对应`,
    status: "active",
    progress: 57,
  },
  {
    id: "bi-alpha-3",
    projectId: "p-alpha",
    blueprintId: "bp-alpha-1",
    code: "A03",
    filePath: "docs/faq.md",
    title: "常见问题",
    goal: "沉淀实施期和交付期最常见的问题与排障建议。",
    summary: "FAQ 问题池已整理，待输出正式问答结构。",
    briefingMarkdown: `# FAQ 编写要求

## 目标
把客户最常问、最容易卡住的问题整理成可直接检索的问答集合。

## 建议结构
- 使用“问题 / 结论 / 处理方式”三段式
- 优先覆盖安装、权限、配置、验收四类问题
- 每个问题保持独立，不要写成长段说明

## 当前约束
- 未确认的问题先不要写结论
- 内部排查记录不要直接暴露给客户`,
    status: "planned",
    progress: 16,
  },
  {
    id: "bi-beta-1",
    projectId: "p-beta",
    blueprintId: "bp-beta-1",
    code: "B01",
    filePath: "ops/approval-flow.md",
    title: "审批链路说明",
    goal: "说明 SOP 从提交、审批到发布的责任链和节点要求。",
    summary: "当前卡在责任边界，不能继续往下写。",
    briefingMarkdown: `# 审批链路说明要求

## 先回答的问题
谁发起、谁审批、谁发布、谁兜底。

## 需要明确的章节
- 流程入口
- 审批角色与职责
- 节点产出物
- 逾期或驳回后的处理方式

## 风险提示
- 如果责任边界尚未拍板，不要伪造完整流程
- 允许标出待管理者确认的空缺`,
    status: "active",
    progress: 39,
  },
  {
    id: "bi-beta-2",
    projectId: "p-beta",
    blueprintId: "bp-beta-1",
    code: "B02",
    filePath: "ops/role-matrix.md",
    title: "角色职责矩阵",
    goal: "定义各岗位在 SOP 生命周期中的输入、输出与责任边界。",
    summary: "矩阵骨架已起草，但依赖审批流裁决结果。",
    briefingMarkdown: `# 角色职责矩阵要求

## 输出形式
以角色为主轴展开，不要按系统模块拆分。

## 至少包含
- 角色名称
- 负责输入
- 负责输出
- 协作对象
- 不负责的边界

## 编写约束
- 如果职责依赖审批流结果，明确引用上游决议
- 不要让两个角色承担同一最终责任`,
    status: "planned",
    progress: 21,
  },
  {
    id: "bi-gamma-1",
    projectId: "p-gamma",
    blueprintId: "bp-gamma-1",
    code: "C01",
    filePath: "kb/knowledge-index.md",
    title: "知识域索引",
    goal: "建立知识域入口、导航和主题分层。",
    summary: "索引首版已完成并落盘。",
    briefingMarkdown: `# 知识域索引要求

## 目标
让新成员在最短时间内找到入口、主题域和关键资料。

## 索引层次
- 一级主题域
- 二级主题分组
- 每组的核心资料入口

## 维护要求
- 新增主题时必须能落入既有层级
- 过时索引先标记，不立即删除`,
    status: "done",
    progress: 100,
  },
  {
    id: "bi-gamma-2",
    projectId: "p-gamma",
    blueprintId: "bp-gamma-1",
    code: "C02",
    filePath: "kb/update-policy.md",
    title: "更新节奏说明",
    goal: "定义知识库维护频率、校验责任和下线策略。",
    summary: "维护节奏已出草案，等待下轮指令。",
    briefingMarkdown: `# 更新节奏说明要求

## 需要回答
谁来更新、多久更新一次、什么情况下下线旧内容。

## 建议章节
- 更新频率
- 校验责任
- 失效判定
- 下线与归档

## 写作限制
- 规则要可执行，避免“定期”“及时”这类空话
- 尽量给出明确触发条件`,
    status: "active",
    progress: 66,
  },
];

export const mockTaskNodes: TaskNode[] = [
  {
    id: "task-alpha-1",
    projectId: "p-alpha",
    blueprintItemId: "bi-alpha-1",
    parentId: null,
    title: "生成交付清单草案",
    summary: "围绕 A01 输出客户可见的目录和附件范围。",
    briefingMarkdown: `# 任务执行要求

## 当前任务
先产出交付清单草案，把客户最终会收到的主文档、附件和配套说明列全。

## 输出要求
- 目录结构先完整，再补每个文档的用途
- 附件要按交付包内外做边界区分
- 不确定项显式标记为待确认

## 上游约束
- 继承 A01 的客户交付口径
- 不得把内部校对材料写进正式交付包`,
    owner: "Executor",
    status: "running",
    progress: 82,
  },
  {
    id: "task-alpha-1-1",
    projectId: "p-alpha",
    blueprintItemId: "bi-alpha-1",
    parentId: "task-alpha-1",
    title: "整理客户访谈摘要",
    summary: "已提取共识、分歧与未决事项。",
    briefingMarkdown: `# 材料梳理要求

## 任务目的
把已有输入材料分成可直接复用、需要改写、待确认三类。

## 输出形式
- 共识信息
- 待确认事项
- 不进入交付包的内部材料

## 注意
- 不要在这里直接写成交付文稿
- 只做归类和判断依据整理`,
    owner: "Planner",
    status: "done",
    progress: 100,
  },
  {
    id: "task-alpha-1-2",
    projectId: "p-alpha",
    blueprintItemId: "bi-alpha-1",
    parentId: "task-alpha-1",
    title: "编排客户可见目录结构",
    summary: "正在整合交付大纲和附件分组。",
    briefingMarkdown: `# 目录整理要求

## 需要完成
把主文档、附件、示例文件组织成稳定目录树。

## 输出重点
- 一级目录命名清晰
- 附件与正文分组明确
- 目录顺序符合客户阅读习惯

## 风险
- 如果附件边界未确认，不要提前锁死最终结构`,
    owner: "Executor",
    status: "running",
    progress: 76,
  },
  {
    id: "task-alpha-1-3",
    projectId: "p-alpha",
    blueprintItemId: "bi-alpha-1",
    parentId: "task-alpha-1",
    title: "确认附件范围",
    summary: "需拍板是否纳入内部校对记录。",
    briefingMarkdown: `# 决策确认项

## 需要拍板
内部校对记录是否作为客户交付附件的一部分。

## 决策前要准备
- 纳入后的收益和风险
- 不纳入时对验收的影响
- 是否存在可替代摘要

## 当前动作
- 暂停继续生成最终目录
- 等待管理者明确结论`,
    owner: "Reviewer",
    status: "waiting_checkpoint",
    progress: 48,
  },
  {
    id: "task-alpha-2",
    projectId: "p-alpha",
    blueprintItemId: "bi-alpha-2",
    parentId: null,
    title: "整理部署文档章节",
    summary: "围绕部署顺序、环境变量和回滚说明产出 A02。",
    briefingMarkdown: `# 部署章节任务要求

## 目标
把部署步骤按执行顺序写清楚，面向实施方，不面向研发内部。

## 必须覆盖
- 前置环境检查
- 配置与环境变量
- 启动验证
- 回滚路径

## 写法限制
- 每一步都要有执行条件
- 不要只贴命令，不解释用途`,
    owner: "Executor",
    status: "running",
    progress: 57,
  },
  {
    id: "task-alpha-2-1",
    projectId: "p-alpha",
    blueprintItemId: "bi-alpha-2",
    parentId: "task-alpha-2",
    title: "收集环境变量清单",
    summary: "正在核对 Helm Chart 和部署模板。",
    briefingMarkdown: `# 配置核对要求

## 当前任务
核对现有部署模板与最终交付文档是否一致。

## 关注点
- 变量名称是否需要对外解释
- 默认值是否安全
- 是否存在内部专用配置暴露风险

## 输出
- 可直接进入文档的配置项
- 需要隐藏或改写的配置项`,
    owner: "Planner",
    status: "running",
    progress: 54,
  },
  {
    id: "task-alpha-2-2",
    projectId: "p-alpha",
    blueprintItemId: "bi-alpha-2",
    parentId: "task-alpha-2",
    title: "补齐回滚说明",
    summary: "回滚章节暂未开始。",
    briefingMarkdown: `# 回滚章节要求

## 任务目的
为部署文档补齐失败后的恢复路径。

## 需要交代
- 回滚触发条件
- 回滚步骤
- 回滚后验证项

## 当前状态
- 依赖前置部署步骤稳定后再写`,
    owner: "Executor",
    status: "backlog",
    progress: 0,
  },
  {
    id: "task-beta-1",
    projectId: "p-beta",
    blueprintItemId: "bi-beta-1",
    parentId: null,
    title: "确认审批责任人",
    summary: "若不先确认责任归属，后续矩阵和 SOP 全部失真。",
    briefingMarkdown: `# 责任链梳理要求

## 当前任务
先把审批链中的最终责任人找准，再继续写流程文档。

## 产出形式
- 责任链草图
- 角色冲突点
- 需要管理者裁决的问题

## 限制
- 未确认前不要输出正式流程版本`,
    owner: "Reviewer",
    status: "blocked",
    progress: 39,
  },
  {
    id: "task-beta-1-1",
    projectId: "p-beta",
    blueprintItemId: "bi-beta-1",
    parentId: "task-beta-1",
    title: "收集现有审批链资料",
    summary: "历史流程材料已整理完毕。",
    briefingMarkdown: `# 历史材料整理要求

## 目的
把旧版本 SOP 中仍有参考价值的内容摘出来，供新版重写引用。

## 输出
- 可复用段落
- 已失效内容
- 与当前职责链冲突的旧描述`,
    owner: "Planner",
    status: "done",
    progress: 100,
  },
  {
    id: "task-beta-1-2",
    projectId: "p-beta",
    blueprintItemId: "bi-beta-1",
    parentId: "task-beta-1",
    title: "裁决跨部门责任边界",
    summary: "等待管理者作出方向决定。",
    briefingMarkdown: `# 等待裁决节点

## 当前情况
新版文档结构依赖管理者在职责拆分方案中拍板。

## 已准备内容
- 两套职责分法
- 各方案影响点
- 待确认风险

## 暂停规则
- 不继续细化下游章节`,
    owner: "Reviewer",
    status: "waiting_checkpoint",
    progress: 18,
  },
  {
    id: "task-gamma-1",
    projectId: "p-gamma",
    blueprintItemId: "bi-gamma-1",
    parentId: null,
    title: "汇总历史规范文档",
    summary: "已产出索引首版并完成归档。",
    briefingMarkdown: `# 索引入口任务要求

## 目标
建立知识域总入口，让不同主题都能被快速定位。

## 输出重点
- 一级主题入口
- 主题间跳转关系
- 关键资料链接

## 当前结果
- 首版已完成，可进入复查阶段`,
    owner: "Executor",
    status: "done",
    progress: 100,
  },
  {
    id: "task-gamma-2",
    projectId: "p-gamma",
    blueprintItemId: "bi-gamma-2",
    parentId: null,
    title: "形成更新节奏草案",
    summary: "维护频率和角色责任已初步收敛。",
    briefingMarkdown: `# 更新节奏任务要求

## 当前任务
给知识库定义可执行的更新频率和责任分工。

## 需要交付
- 更新触发条件
- 责任人
- 过期内容下线规则

## 写法要求
- 尽量使用明确时间或触发条件
- 避免“定期维护”这种空话`,
    owner: "Planner",
    status: "running",
    progress: 66,
  },
];

export const mockProjectEvents: ProjectEvent[] = [
  {
    id: "event-alpha-1",
    projectId: "p-alpha",
    kind: "decision",
    title: "打开审议：附件范围待拍板",
    detail: "来源于 A01 的检查点，等待确认是否纳入内部校对记录。",
    at: "18:22",
    tone: "warning",
  },
  {
    id: "event-alpha-2",
    projectId: "p-alpha",
    kind: "output",
    title: "产出更新：delivery-outline.md",
    detail: "执行智能体完成交付目录草案写入。",
    at: "18:20",
    tone: "success",
  },
  {
    id: "event-alpha-3",
    projectId: "p-alpha",
    kind: "task",
    title: "A02 根任务开始运行",
    detail: "部署文档和环境变量说明进入执行阶段。",
    at: "18:17",
    tone: "primary",
  },
  {
    id: "event-alpha-4",
    projectId: "p-alpha",
    kind: "blueprint",
    title: "蓝图重新批准",
    detail: "已确认部署文档必须包含环境变量章节。",
    at: "18:08",
    tone: "neutral",
  },
  {
    id: "event-beta-1",
    projectId: "p-beta",
    kind: "decision",
    title: "责任边界待裁决",
    detail: "B01 阻塞，角色矩阵无法继续下钻。",
    at: "17:48",
    tone: "warning",
  },
  {
    id: "event-beta-2",
    projectId: "p-beta",
    kind: "blueprint",
    title: "蓝图进入 revised",
    detail: "用户调整了审批顺序要求，待重新批准。",
    at: "17:30",
    tone: "neutral",
  },
  {
    id: "event-gamma-1",
    projectId: "p-gamma",
    kind: "output",
    title: "knowledge-index.md 已落盘",
    detail: "知识域索引首版完成，进入等待下一轮指令状态。",
    at: "昨天 18:20",
    tone: "success",
  },
  {
    id: "event-gamma-2",
    projectId: "p-gamma",
    kind: "project",
    title: "项目进度更新为 83%",
    detail: "索引与更新节奏两条主线均已形成可评审版本。",
    at: "昨天 17:55",
    tone: "primary",
  },
];

export const mockConversations: ProjectConversation[] = [
  {
    id: "conv-alpha",
    projectId: "p-alpha",
    title: "客户 A 交付包对话",
    updatedAt: "刚刚",
  },
  {
    id: "conv-beta",
    projectId: "p-beta",
    title: "内部 SOP 对话",
    updatedAt: "17:48",
  },
  {
    id: "conv-gamma",
    projectId: "p-gamma",
    title: "知识库整理对话",
    updatedAt: "昨天 18:40",
  },
];

export const mockMessages: ProjectMessage[] = [
  {
    id: "msg-alpha-1",
    projectId: "p-alpha",
    role: "user",
    content: "现在这个界面感觉还是传统交互的方式，我想做成智能体优先的交互方式，如果重新设计这个界面，你会怎么设计。",
    at: "18:04",
  },
  {
    id: "msg-alpha-2",
    projectId: "p-alpha",
    role: "agent",
    agentId: "agent-planner",
    agentName: "规划智能体",
    agentKind: "planner",
    handledDuration: "00:30",
    content: "我会先把交互方式拆成三层：项目总览、项目内持续对话、右侧工作区监看。重点不是把右侧跟着聊天乱跳，而是把多智能体的推进状态持续露出来。",
    at: "18:05",
    activities: [
      {
        kind: "thought",
        label: "思考结束",
        durationLabel: "1s",
        body: "正在把「多项目并行 + 跨项目进度概览」写入设计文档，并与现有项目中心、智能体优先布局衔接。",
      },
      {
        kind: "tool",
        label: "查看文件",
        durationLabel: "2s",
        body: "核对设计文档与现有前端壳层，确认项目栏、对话栏、工作区三列的职责边界。",
        relatedFiles: [
          "docwise-ui-shell-design.md  +1 -1",
          "docwise-ui-agent-first-design.md  +2 -0",
        ],
      },
    ],
  },
  {
    id: "msg-alpha-3",
    projectId: "p-alpha",
    role: "agent",
    agentId: "agent-executor",
    agentName: "执行智能体",
    agentKind: "executor",
    handledDuration: "00:18",
    content: "我会把中列改成统一的消息块结构：用户和智能体都靠左，执行轨迹只挂在对应智能体下面，避免整段内容被塞进一个臃肿的大气泡里。",
    at: "18:07",
  },
  {
    id: "msg-alpha-4",
    projectId: "p-alpha",
    role: "user",
    content: "现在的模式只能同时处理单个项目，但是有了智能体之后一般都是同时处理多个项目，有时候还想看到整体项目的进度概览之类的。",
    at: "18:08",
  },
  {
    id: "msg-alpha-5",
    projectId: "p-alpha",
    role: "agent",
    agentId: "agent-coordinator",
    agentName: "协调智能体",
    agentKind: "coordinator",
    handledDuration: "00:12",
    content: "这个诉求应该放到左侧项目栏之上或之内，用全局项目队列去表达；中列仍然只承接当前项目对话，但要明确告诉你此刻是哪个智能体在发言、谁在推进、谁在等待裁决。",
    at: "18:09",
  },
  {
    id: "msg-beta-1",
    projectId: "p-beta",
    role: "user",
    content: "先不要写细节，先把审批责任人与边界说清楚。",
    at: "17:20",
  },
  {
    id: "msg-beta-2",
    projectId: "p-beta",
    role: "agent",
    agentId: "agent-reviewer",
    agentName: "审阅智能体",
    agentKind: "reviewer",
    handledDuration: "00:09",
    content: "已把 B01 标成阻塞。当前需要你裁决跨部门审批的最终责任归属。",
    at: "17:48",
  },
  {
    id: "msg-gamma-1",
    projectId: "p-gamma",
    role: "agent",
    agentId: "agent-executor",
    agentName: "执行智能体",
    agentKind: "executor",
    handledDuration: "00:24",
    content: "索引首版已完成，接下来可以继续扩写更新节奏，或者转入下一批知识域清理。",
    at: "昨天 18:40",
  },
];
