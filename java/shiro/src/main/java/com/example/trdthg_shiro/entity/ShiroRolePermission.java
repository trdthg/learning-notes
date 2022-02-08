package com.example.trdthg_shiro.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableId;
import java.io.Serializable;
import lombok.Data;
import lombok.EqualsAndHashCode;

/**
 * <p>
 * 
 * </p>
 *
 * @author trdthg
 * @since 2021-05-29
 */
@Data
@EqualsAndHashCode(callSuper = false)
public class ShiroRolePermission implements Serializable {

    private static final long serialVersionUID = 1L;

    @TableId(value = "id", type = IdType.AUTO)
    private Integer id;

    private String remarks;

    private Integer roleId;

    private Integer permissionId;


}
