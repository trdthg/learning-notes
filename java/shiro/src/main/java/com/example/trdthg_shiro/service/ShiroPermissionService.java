package com.example.trdthg_shiro.service;

import com.example.trdthg_shiro.entity.ShiroPermission;
import com.baomidou.mybatisplus.extension.service.IService;
import com.example.trdthg_shiro.mapper.ShiroPermissionMapper;
import org.springframework.beans.factory.annotation.Autowired;

import java.util.List;

/**
 * <p>
 *  服务类
 * </p>
 *
 * @author trdthg
 * @since 2021-05-29
 */
public interface ShiroPermissionService extends IService<ShiroPermission> {

    List<String> findByUserId(Integer id);

}
