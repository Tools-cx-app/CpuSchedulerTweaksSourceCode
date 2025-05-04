# life-death-scheduler

## 调度简介

> 本配置文件用于定义不同电源策略下的 CPU 核心频率参数，包含通用模式设置、CPU 策略分配、频率控制参数和应用专属配置。

## 配置文件

### 1. 全局模式设置 (osm)

```toml
osm = "powersave"
```

- **作用**：定义默认应用的电源策略（applist 指定的应用除外）
- **可选值**：`powersave`/`balance`/`performance`/`fast`

### 2. CPU 策略分配 (cpu_config)

```toml
[cpu_config]
big = 7
middle = 4
small = 0
```

- **作用**：定义不同策略类型对应的 CPU 核心编号
- **字段说明**：
  - `big`: 大核 CPU 编号上限（核心 0-7）
  - `middle`: 中核 CPU 编号上限（核心 0-4）
  - `small`: 小核 CPU 编号上限（核心 0）

### 3. 电源策略配置

#### 3.1 节能模式 (powersave)

```toml
[powersave]
big_cpu_freq = { max = 1800000, min = 500000 }   # 1.8GHz ~ 0.5GHz
middle_cpu_freq = { max = 1600000, min = 500000 } # 1.6GHz ~ 0.5GHz
small_cpu_freq = { max = 1400000, min = 500000 }  # 1.4GHz ~ 0.5GHz
```

#### 3.2 均衡模式 (balance)

```toml
[balance]
big_cpu_freq = { max = 2200000, min = 800000 }   # 2.2GHz ~ 0.8GHz
middle_cpu_freq = { max = 2000000, min = 800000 } # 2.0GHz ~ 0.8GHz
small_cpu_freq = { max = 1800000, min = 800000 }  # 1.8GHz ~ 0.8GHz
```

#### 3.3 性能模式 (performance)

```toml
[performance]
big_cpu_freq = { max = 2800000, min = 1200000 }  # 2.8GHz ~ 1.2GHz
middle_cpu_freq = { max = 2500000, min = 1200000 } # 2.5GHz ~ 1.2GHz
small_cpu_freq = { max = 2200000, min = 1200000 } # 2.2GHz ~ 1.2GHz
```

#### 3.4 极限模式 (fast)

```toml
[fast]
big_cpu_freq = { max = 3200000, min = 1500000 }  # 3.2GHz ~ 1.5GHz
middle_cpu_freq = { max = 2800000, min = 1500000 } # 2.8GHz ~ 1.5GHz
small_cpu_freq = { max = 2500000, min = 1500000 } # 2.5GHz ~ 1.5GHz
```

### 4. 应用专属配置 (applist)

```toml
[applist]
"bin.mt.plus" = "powersave"
```

- **作用**：为指定应用设置专属电源策略
- **格式**：`"应用包名" = "策略模式"`
- **示例说明**：当启动 `bin.mt.plus`（MT 管理器）时，自动切换为节能模式

## 频率单位说明

所有频率参数单位为 **赫兹(Hz)**，配置时需注意：

- 1 kHz = 1,000 Hz
- 1 MHz = 1,000,000 Hz
- 1 GHz = 1,000,000,000 Hz

## 策略优先级

1. applist 应用专属配置
2. osm 全局默认配置
