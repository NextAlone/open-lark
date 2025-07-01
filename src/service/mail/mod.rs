//! 邮箱（Mail）服务
//!
//! 提供飞书邮箱的完整功能集，支持邮件管理、邮箱群组、联系人管理、
//! 事件处理等企业级邮件服务能力。是企业邮件通信和协作的核心工具。
//!
//! # 核心功能
//!
//! ## 邮件管理
//! - 📧 邮件发送、接收和管理
//! - 📋 邮件列表查询和筛选
//! - 📎 附件上传和下载
//! - 🔍 邮件搜索和检索
//! - 📊 邮件状态跟踪
//!
//! ## 邮箱群组管理
//! - 👥 邮箱群组创建和管理
//! - 📝 群组成员添加和移除
//! - ⚙️ 群组设置和权限配置
//! - 📋 群组信息查询和更新
//! - 👑 群组管理员设置
//!
//! ## 联系人管理
//! - 📇 邮箱联系人管理
//! - 👥 联系人信息查询和更新
//! - 📋 联系人分组和标签
//! - 🔍 联系人搜索和筛选
//! - 📊 联系人使用统计
//!
//! ## 事件管理
//! - 📅 邮件事件监听和处理
//! - 🔔 事件通知和推送
//! - 📊 事件状态跟踪
//! - 🔄 事件重试和容错
//! - 📋 事件日志记录
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取邮箱服务
//! let mail = &client.mail;
//!
//! // 发送邮件
//! // let message_request = SendMessageRequest::builder()
//! //     .to(vec!["user@company.com"])
//! //     .subject("重要通知")
//! //     .content("这是一封重要的企业邮件")
//! //     .build();
//! // mail.v1.message.send(message_request, None).await?;
//!
//! // 查询邮件列表
//! // let list_request = ListMessagesRequest::builder()
//! //     .folder("inbox")
//! //     .page_size(20)
//! //     .build();
//! // let messages = mail.v1.message.list(list_request, None).await?;
//!
//! // 创建邮箱群组
//! // let group_request = CreateMailGroupRequest::builder()
//! //     .name("项目团队")
//! //     .email("project-team@company.com")
//! //     .description("项目团队邮箱群组")
//! //     .build();
//! // mail.v1.mail_group.create(group_request, None).await?;
//!
//! // 添加群组成员
//! // let member_request = AddMailGroupMemberRequest::builder()
//! //     .group_id("group_123")
//! //     .members(vec!["user1@company.com", "user2@company.com"])
//! //     .build();
//! // mail.v1.mail_group_manager.add_members(member_request, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v1版本，提供完整的邮箱功能：
//! - 邮件全生命周期管理
//! - 邮箱群组和权限管理
//! - 联系人和地址簿
//! - 事件处理和通知
//!
//! # 邮箱特性
//!
//! - 📧 企业级邮件安全
//! - 🔐 加密传输和存储
//! - 📱 多平台同步支持
//! - 🔍 智能搜索和分类
//! - 📊 邮件统计和分析
//!
//! # 集成能力
//!
//! - 📅 日历系统集成
//! - 👥 通讯录同步
//! - 📋 工作流集成
//! - 🔔 多渠道通知
//! - 📊 数据分析和报表

/// 数据模型定义
pub mod models;
/// 邮箱服务 v1 版本
pub mod v1;

use crate::core::config::Config;

/// 邮箱服务
///
/// 企业级邮件服务的统一入口，提供邮件管理、邮箱群组、
/// 联系人管理、事件处理等完整的邮件服务能力。
///
/// # 服务架构
///
/// - **v1**: 邮箱API v1版本，提供完整功能集
/// - **models**: 数据模型和结构定义
///
/// # 核心特性
///
/// - 📧 全面的邮件管理功能
/// - 👥 灵活的群组管理系统
/// - 📇 完善的联系人管理
/// - 📅 智能的事件处理
/// - 🔐 企业级安全保障
///
/// # 适用场景
///
/// - 企业内部邮件通信
/// - 团队协作和沟通
/// - 客户邮件营销
/// - 邮件自动化处理
/// - 邮件数据分析
///
/// # 最佳实践
///
/// - 合理设置邮箱群组
/// - 定期清理邮件数据
/// - 监控邮件发送状态
/// - 保护邮件隐私安全
/// - 建立邮件规范流程
pub struct MailService {
    /// v1版本API服务
    pub v1: v1::V1,
}

impl MailService {
    /// 创建新的邮箱服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的邮箱服务实例
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}
